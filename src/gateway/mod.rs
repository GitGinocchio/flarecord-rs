use std::{mem::forget, sync::Arc};
use futures::{TryStreamExt, lock::Mutex};
use serde::de::DeserializeSeed;
use serde_json::json;
use twilight_model::gateway::{
    Intents, event::{DispatchEvent, EventType, GatewayEvent, GatewayEventDeserializer}, payload::{incoming::Hello, outgoing::{
        Heartbeat, 
        Identify, 
        Resume, 
        identify::{
            IdentifyInfo, 
            IdentifyProperties
        },
    }}
};
use wasm_bindgen::JsCast;
use worker::{wasm_bindgen::JsValue, *};

use crate::gateway::{constants::{ALARM_FALLBACK_DELAY_MS, CREDENTIALS_KEY, GATEWAY_BOT_URL, GATEWAY_INTENTS, GATEWAY_VERSION, RECONNECT_RATE_LIMIT, RECONNECT_RATE_WINDOW_MS, STATE_KEY, WEBHOOK_MAX_ATTEMPTS, is_forwarded_event_type}, credentials::GatewayCredentials, handle::GatewayHandle, inner::GatewayInner, state::GatewayState, status::{GatewayStatus, Status}, utils::{CloseAction, ConnectError, GatewayBotResponse, GatewayError, GatewayInfo, OpenWebSocketError, ReconnectOptions, ReconnectStrategy, can_resume, classify_close_code, is_private_hostname, to_http_url}};

pub mod credentials;
pub mod constants;
pub mod status;
pub mod state;
pub mod inner;
pub mod utils;
pub mod handle;
pub mod bridge;


#[durable_object]
pub struct DiscordGateway {
    inner: Arc<GatewayInner>,
}

impl DurableObject for DiscordGateway {
    fn new(state: State, env: Env) -> Self {
        Self {
            inner: Arc::new(GatewayInner::new(env, state)),
        }
    }

    async fn fetch(&self, req: Request) -> Result<Response> {
        let env = self.inner.env.clone();

        Router::new()
            .get_async("/status", async |_, _| self.http_get_status().await)
            .post_async("/connect", async |req, ctx| self.http_post_connect(req, ctx).await)
            .post_async("/disconnect", async |req, ctx| self.http_post_disconnect(req, ctx).await)
            .run(req, env)
            .await
    }

    async fn alarm(&self) -> Result<Response, Error> {
        match DiscordGateway::alarm_internal(&self.as_handle()).await {
            Ok(_) => Response::empty(),
            Err(e) => {
                // Logghiamo l'errore
                console_error!("discord-gateway: alarm handler failed: {:?}", e);
                
                // Reschedule forzato: garantisce che il loop dell'allarme non si fermi
                // mai, evitando la perdita permanente dell'allarme post-6 retries.
                let target_time_ms = js_sys::Date::now() + ALARM_FALLBACK_DELAY_MS as f64;
                let date = js_sys::Date::new(&target_time_ms.into());

                let storage_guard = self.inner.storage.lock().await;
                storage_guard.set_alarm(ScheduledTime::new(date)).await?;
                
                Response::empty()
            }
        }
    }
}

impl DiscordGateway {
    pub async fn http_get_status(&self) -> Result<Response> {
        let status = self.status().await?;
        
        Response::from_json(&json!({
            "connected_at" : status.connected_at,
            "reconnect_attempts" : status.reconnect_attempts,
            "status" : status.status
        }))
    }

    pub async fn http_post_connect(&self, mut req: Request, _ctx: RouteContext<()>) -> Result<Response> {
        let credentials: GatewayCredentials = match req.json().await {
            Err(e) => return Response::error(format!("Request body missing or malformed: {e}"), 400),
            Ok(b) => b
        };

        if let Some(state) = self.load_state().await? && state.connected_at.is_some() {
            return Response::from_json(&json!({
                "message" : "Gateway already connected"
            }))
        }

        let message = self.connect(credentials).await?;
        Response::from_json(&json!({
            "message" : message,
        }))
    }

    pub async fn http_post_disconnect(&self, _req: Request, _ctx: RouteContext<()>) -> Result<Response> {
        let message = self.disconnect().await?;
        Response::from_json(&json!({
            "message" : message,
        }))
    }
}

impl DiscordGateway {
    /// Connects to the Discord Gateway.
    ///
    /// Stores credentials and initiates the WebSocket connection.
    ///
    /// The connection is established asynchronously — this method returns
    /// once the WebSocket is opened, but the Discord `READY` handshake
    /// may still be in progress. Poll `status()` to confirm the
    /// connection is fully established.
    ///
    /// # Returns
    ///
    /// Returns `Ok(String)` containing `"connecting"` on success, 
    /// or an `Err(String)` containing the error message on failure.
    async fn connect(&self, creds: GatewayCredentials) -> Result<String, String> {
        if creds.bot_token.is_empty() {
            return Err("Bot Token is required".to_string());
        }

        if let Some(webhook_url) = &creds.webhook_url {
            // 2. Validazione URL
            let parsed_url = Url::parse(webhook_url)
                .map_err(|_| "webhookUrl must be a valid URL".to_string())?;

            if parsed_url.scheme() != "https" {
                return Err("webhookUrl must use HTTPS".to_string());
            }
            if !parsed_url.username().is_empty() || !parsed_url.password().unwrap_or("").is_empty() {
                return Err("webhookUrl must not contain credentials".to_string());
            }

            if is_private_hostname(parsed_url.host_str().unwrap_or("")) {
                return Err("webhookUrl host must be publicly routable".to_string());
            }
        }

        // 3. Persistenza
        let stored = GatewayCredentials {
            bot_token: creds.bot_token,
            webhook_url: creds.webhook_url,
            webhook_secret: creds.webhook_secret,
        };

        {
            let storage_guard = self.inner.storage.lock().await;
            storage_guard.put(CREDENTIALS_KEY, &stored)
                .await
                .map_err(|e| e.to_string())?;

            let mut creds = self.inner.cached_credentials.lock().await;
            *creds = Some(stored);
        }

        // 4. Reset stato riconnessione
        let mut existing_state = self.load_state()
            .await
            .map_err(|e| e.to_string())?
            .unwrap_or_default();
        existing_state.reconnect_disabled = false;
        self.save_state(&existing_state).await.map_err(|e| e.to_string())?;

        let mut reconnect_disabled = self.inner.reconnect_disabled.lock().await;
        *reconnect_disabled = false;

        let handle = self.as_handle();

        DiscordGateway::connect_internal(&handle)
            .await
            .expect("Error connecting to Discord via websocket: ");

        Ok("connecting".into())
    }

    /// Disconnect from the Discord Gateway.
    /// Closes the WebSocket and clears all state.
    async fn disconnect(&self) -> Result<String, String> {
        // 1. Chiudi internamente la connessione (WebSocket)
        // Assicurati che disconnect_internal gestisca il close del WebSocketPair
        self.disconnect_internal().await.map_err(|e| e.to_string())?;

        let guard = self.inner.storage.lock().await;

        // 2. Elimina le credenziali dallo storage persistente
        guard
            .delete(CREDENTIALS_KEY)
            .await
            .map_err(|e| e.to_string())?;

        let mut guard = self.inner.cached_credentials.lock().await;
        *guard = None;

        // 4. Restituisci lo stato di successo
        Ok("disconnected".to_string())
    }

    /// Get the current Gateway connection status.
    async fn status(&self) -> Result<GatewayStatus, String> {
        let state = self.load_state()
            .await
            .map_err(|e| e.to_string())?
            .unwrap_or_else(|| GatewayState::default());

        let guard = self.inner.upstream.lock().await;
        
        // Logica per determinare lo stato corrente
        let status_str = if guard.is_some() {
            Status::Connected
        } else if state.ws_url.is_some() {
            Status::Connecting
        } else {
            Status::Disconnected
        };

        Ok(GatewayStatus {
            status: status_str,
            session_id: state.session_id,
            connected_at: state.connected_at,
            sequence: state.sequence,
            reconnect_attempts: state.reconnect_attempts,
        })
    }

    async fn alarm_internal(handle: &GatewayHandle) -> Result<(), worker::Error> {
        {
            let guard = handle.inner.reconnect_disabled.lock().await;
            if *guard {
                return Ok(());
            }
        }

        let mut state = match DiscordGateway::load_state_from_handle(handle).await? {
            Some(s) => s,
            None => {
                // Se non c'è stato, tenta di recuperare le credenziali e riconnettersi
                if let Some(_creds) = DiscordGateway::load_credentials_from_handle(handle).await? {
                    DiscordGateway::connect_internal(handle).await?;
                }
                return Ok(());
            }
        };

        // 2. Stato terminale
        if state.reconnect_disabled {
            let mut guard = handle.inner.reconnect_disabled.lock().await;
            *guard = true;

            let guard = handle.inner.storage.lock().await;
            guard.delete_alarm().await?;
            return Ok(());
        }

        // 3. Cooldown Identify
        let now = js_sys::Date::now();
        if let Some(cooldown) = state.identify_cooldown_until {
            if now < cooldown {
                let date = js_sys::Date::new(&(now + cooldown).into());
                let guard = handle.inner.storage.lock().await;
                guard.set_alarm(ScheduledTime::new(date))
                    .await?;
                return Ok(());
            }
        }

        // 4. Riconnessione se manca wsUrl
        if state.ws_url.is_none() {
            DiscordGateway::connect_internal(handle).await?;
            return Ok(());
        }

        let upstream_is_none = {
            let guard = handle.inner.upstream.lock().await;
            guard.is_none()
        };

        // 5. DO Eviction: WebSocket reference persa
        if upstream_is_none {
            console_warn!("discord-gateway: WebSocket reference lost (DO eviction); reconnecting");
            state.ws_url = None;
            state.heartbeat_interval_ms = None;
            DiscordGateway::save_state_from_handle(handle, &state).await?;
            DiscordGateway::connect_internal(handle).await?;
            return Ok(());
        }

        // 6. Sessione invalida (manca heartbeat)
        if state.heartbeat_interval_ms.is_none() {
            DiscordGateway::identify_or_resume(handle, &mut state).await?;
            return Ok(());
        }

        // 7. Missed Heartbeat Ack
        let last_ack = state.last_heartbeat_ack.unwrap_or(0.0);
        let interval = state.heartbeat_interval_ms.unwrap_or(0);
        
        if now - last_ack > (interval as f64 * 2.0) {
            console_warn!("discord-gateway: heartbeat missed; reconnecting");
            let options = ReconnectOptions {
                strategy: Some(ReconnectStrategy::ResumeOrIdentify),
                clear_session: false,
                reason: Some("heartbeat missed".to_string()),
            };
            DiscordGateway::reconnect_with_backoff(handle, Some(options)).await?;
            return Ok(());
        }

        // 8. Heartbeat invio e scheduling
        DiscordGateway::send_heartbeat(handle, &state).await?;
        DiscordGateway::schedule_heartbeat(handle, &state).await?;

        Ok(())
    }

    async fn connect_internal(handle: &GatewayHandle) -> Result<(), ConnectError>  {
        let mut state = DiscordGateway::load_state_from_handle(handle).await
            .map_err(|e| ConnectError { error: e.to_string(), retry_scheduled: false })?
            .unwrap_or_default();

        // 1. Già connesso o terminale
        {

            let guard = handle.inner.upstream.lock().await;
            if guard.is_some() {
                return Ok(())
            }
        }

        if state.reconnect_disabled {
            return Err(ConnectError {
                error: "reconnect disabled after terminal close code".to_string(),
                retry_scheduled: false,
            });
        }

        // 2. Caricamento credenziali
        let creds = match DiscordGateway::load_credentials_from_handle(handle).await {
            Ok(Some(c)) => c,
            _ => {
                let error = "no credentials stored; cannot connect".to_string();
                console_error!("discord-gateway: {}", error);
                return Err(ConnectError { error, retry_scheduled: false });
            }
        };

        // 3. Risoluzione Gateway URL
        let resumable = can_resume(&state);

        if state.ws_url.is_none() {
            match DiscordGateway::get_gateway_info(&creds.bot_token).await {
                Ok(info) => {
                    state.ws_url = Some(info.url);
                    state.session_start_remaining = Some(info.session_start_limit.remaining);
                    state.session_start_reset_after_ms = Some(info.session_start_limit.reset_after);
                    state.session_start_max_concurrency = Some(info.session_start_limit.max_concurrency);
                    state.session_start_total = Some(info.session_start_limit.total);
                    DiscordGateway::save_state_from_handle(handle, &state).await?;
                }
                Err(GatewayError::Api(message, retryable, _status)) => {
                    if retryable {
                        let reconnect_options = ReconnectOptions {
                            strategy: Some(state.reconnect_strategy),
                            clear_session: false,
                            reason: Some(message.clone())
                        };

                        DiscordGateway::reconnect_with_backoff(handle, Some(reconnect_options)).await?;
                        return Err(ConnectError { error: message, retry_scheduled: true });
                    }
                    return Err(ConnectError { error: message, retry_scheduled: false });
                },
                Err(GatewayError::Network(message)) => {
                    return Err(ConnectError { error: message, retry_scheduled: false });
                }
            }
        }

        // 4. Identify Cooldown / Limiti
        let now = js_sys::Date::now();
        let needs_identify = !can_resume(&state);
        if needs_identify {
            if let Some(cooldown) = state.identify_cooldown_until {
                if now < cooldown {
                    let guard = handle.inner.storage.lock().await;
                    let _ = guard.set_alarm(ScheduledTime::new(js_sys::Date::new(&cooldown.into()))).await;
                    return Err(ConnectError { error: "identify cooldown active".to_string(), retry_scheduled: true });
                }
            }
            
            if state.session_start_remaining.unwrap_or(1) <= 0 {
                if let Some(reset) = state.session_start_reset_after_ms {
                    state.identify_cooldown_until = Some(now + reset as f64);
                    state.ws_url = None;
                    DiscordGateway::save_state_from_handle(handle, &state).await?;
                    let guard = handle.inner.storage.lock().await;
                    let _ = guard.set_alarm(ScheduledTime::new(js_sys::Date::new(&(now + reset as f64).into()))).await;
                    return Err(ConnectError { error: "session start limit exhausted".to_string(), retry_scheduled: true });
                }
            }
        }

        if resumable && state.ws_url.is_none() {
            state.ws_url = state.resume_gateway_url.clone();
        }

        state.heartbeat_interval_ms = None;
        state.last_heartbeat_ack = Some(now);
        state.connected_at = Some(now);
        state.reconnect_disabled = false;

        DiscordGateway::save_state_from_handle(&handle, &state).await?;

        match DiscordGateway::open_websocket(&handle, &state.ws_url.unwrap()).await {
            Ok(_) => Ok(()),
            Err(open_result) => {
                if open_result.retryable {
                    let reconnect_options = ReconnectOptions {
                        strategy: Some(state.reconnect_strategy),
                        clear_session: false,
                        reason: Some(open_result.error.clone())
                    };

                    DiscordGateway::reconnect_with_backoff(&handle, Some(reconnect_options)).await?;
                } else {
                    state.ws_url = None;

                    DiscordGateway::save_state_from_handle(&handle, &state).await?;
                }
                Err(ConnectError { 
                    error: open_result.error, 
                    retry_scheduled: open_result.retryable 
                })
            }
        }
    }

    async fn disconnect_internal(&self) -> Result<(), worker::Error> {
        let mut upstream_guard = self.inner.upstream.lock().await;

        // 2. Gestione WebSocket (upstream)
        if let Some(upstream) = upstream_guard.take() {
            let mut guard = self.inner.suppress_reconnect.lock().await;
            *guard = true;
            
            // Chiudi il WebSocket. In workers-rs, close() è un'operazione che può fallire,
            // quindi usiamo un match o ignore con un '_' se vogliamo silenziare l'errore.
            let _ = upstream.close(Some(1000), Some("client disconnect"));
        }

        // 3. Pulisci lo stato nel Durable Object storage
        let storage_guard = self.inner.storage.lock().await;
        storage_guard.delete(STATE_KEY).await?;
        
        // 4. Reset variabili di stato
        let mut guard = self.inner.reconnect_disabled.lock().await;
        *guard = false;
        
        // 5. Cancella l'allarme (importante per non avere heartbeat residui)
        storage_guard.delete_alarm().await?;

        Ok(())
    }

    async fn reconnect_with_backoff(handle: &GatewayHandle, options: Option<ReconnectOptions>) -> Result<(), worker::Error> {
        let mut state = DiscordGateway::load_state_from_handle(handle).await?.unwrap_or_default();
        let attempts = state.reconnect_attempts + 1;

        // Calcolo Exponential Backoff
        let max_backoff = 60_000.0; // Esempio: 60s
        let delay = (1000.0 * (2.0f64.powi(attempts as i32))).min(max_backoff) + (js_sys::Math::random() * 1000.0);

        // Aggiornamento stato
        state.reconnect_attempts = attempts;
        state.ws_url = None;
        state.heartbeat_interval_ms = None;
        
        if let Some(opts) = options {
            if let Some(strat) = opts.strategy {
                state.reconnect_strategy = strat;
            }
            if opts.clear_session {
                state.session_id = None;
                state.sequence = None;
            }
        }
        state.reconnect_disabled = false;

        DiscordGateway::save_state_from_handle(handle, &state).await?;

        // Chiusura WebSocket (Gestione del borrow_mut
        let mut upstream_guard = handle.inner.upstream.lock().await;
        if upstream_guard.is_some() {
            let mut guard = handle.inner.reconnect_planned.lock().await;
            *guard = true;

            if let Some(ws) = upstream_guard.take() {
                let _ = ws.close(Some(4000), Some("reconnecting")); // 4000 è il codice personalizzato
            }
        }

        console_warn!(
            "discord-gateway: scheduling reconnect; attempts: {}, delay: {}ms",
            attempts,
            delay.round()
        );

        // Setup Allarme
        let schedule = js_sys::Date::new(&(js_sys::Date::now() + delay).into());
        let guard = handle.inner.storage.lock().await;
        guard.set_alarm(ScheduledTime::new(schedule)).await?;

        Ok(())
    }

    /// Schedule a delayed reconnect via alarm instead of blocking with setTimeout.
    /// Used for op 7 (Reconnect) where Discord requires a minimum delay.
    async fn reconnect_with_min_delay(handle: &GatewayHandle, options: Option<ReconnectOptions>) -> Result<(), worker::Error> {
        let mut state = DiscordGateway::load_state_from_handle(handle).await?.unwrap_or_default();
        
        state.ws_url = None;
        state.heartbeat_interval_ms = None;
        
        if let Some(opts) = options {
            if let Some(strat) = opts.strategy {
                state.reconnect_strategy = strat;
            }
            if opts.clear_session {
                state.session_id = None;
                state.sequence = None;
            }
        }
        state.reconnect_disabled = false;

        DiscordGateway::save_state_from_handle(handle, &state).await?;

        // 2. Chiusura WebSocket (Scope limitato per rilasciare il borrow)
        let mut upstream_guard = handle.inner.upstream.lock().await;
        if upstream_guard.is_some() {
            let mut guard = handle.inner.reconnect_planned.lock().await;
            *guard = true;
            
            if let Some(ws) = upstream_guard.take() {
                let _ = ws.close(Some(4000), Some("reconnecting"));
            }
        }

        // 3. Scheduling dell'allarme di riconnessione (1 secondo)
        let delay_ms = 1000.0;
        let schedule = js_sys::Date::new(&(js_sys::Date::now() + delay_ms).into());

        let guard = handle.inner.storage.lock().await;
        guard.set_alarm(ScheduledTime::new(schedule))
            .await?;

        Ok(())
    }

    /// Sliding-window rate limiter: max RECONNECT_RATE_LIMIT reconnects
    /// per RECONNECT_RATE_WINDOW_MS. Prevents runaway reconnect loops.
    /// Note: This counter is in-memory only — it resets on DO eviction.
    /// The persistent `reconnectAttempts` backoff provides the primary
    /// protection across evictions.
    async fn is_reconnect_rate_limited(handle: &GatewayHandle) -> bool {
        let now = js_sys::Date::now();

        // Rimuove i timestamp più vecchi della finestra (in-place)
        let mut guard = handle.inner.reconnect_timestamps.lock().await;
        guard.retain(|&t| now - t < RECONNECT_RATE_WINDOW_MS as f64);

        // Verifica se abbiamo superato il limite
        guard.len() >= RECONNECT_RATE_LIMIT as usize
    }

    async fn identify_or_resume(handle: &GatewayHandle, state: &mut GatewayState) -> Result<(), worker::Error> {
        // 1. Recupero WS e Credenziali

        let ws = {
            let guard = handle.inner.upstream.lock().await;
            guard.clone()
        };

        let ws = match ws {
            Some(w) => w,
            None => return Ok(()),
        };

        let creds = DiscordGateway::load_credentials_from_handle(handle)
            .await?
            .ok_or(worker::Error::from("Missing credentials"))?;
        let bot_token = creds.bot_token;

        // 2. Tenta Resume
        if can_resume(&state) {
            let resume = Resume::new(
                state.sequence.unwrap_or_default(), 
                state.session_id.clone().unwrap_or_default(), 
                bot_token
            );

            ws.send(&resume).map_err(|e| worker::Error::from(e.to_string()))?;
            return Ok(());
        }

        // 3. Gestione Cooldown e Limitazioni
        let now = js_sys::Date::now() as u64;

        if let Some(cooldown) = state.identify_cooldown_until {
            if now < cooldown as u64 {
                DiscordGateway::handle_identify_block(handle, ws, state, "identify cooldown", cooldown as u64).await?;
                return Ok(());
            }
        }

        if let (Some(remaining), Some(reset_after)) = (state.session_start_remaining, state.session_start_reset_after_ms) {
            if remaining <= 0 && reset_after > 0 {
                let cooldown = now + reset_after;
                DiscordGateway::handle_identify_block(handle, ws, state, "session start limit exhausted", cooldown as u64).await?;
                return Ok(());
            }
        }

        let identify = Identify::new(IdentifyInfo {
            compress: false,
            intents: Intents::from_bits(GATEWAY_INTENTS).unwrap_or(Intents::empty()),
            large_threshold: 50, // Soglia per i membri
            presence: None,
            properties: IdentifyProperties::new(
                "discord-gateway-cloudflare-do", 
                "discord-gateway-cloudflare-do",
                "cloudflare"
            ),
            shard: None,
            token: bot_token
        });

        ws.send(&identify).map_err(|e| worker::Error::from(e.to_string()))?;

        // 5. Aggiornamento stato dopo Identify riuscito
        if let Some(ref mut remaining) = state.session_start_remaining {
            if *remaining > 0 { *remaining -= 1; }
        }
        state.identify_cooldown_until = None;
        state.reconnect_strategy = ReconnectStrategy::ResumeOrIdentify;

        DiscordGateway::save_state_from_handle(handle, &state).await?;
        Ok(())
    }

    // Helper per ridurre il boilerplate del blocco identify
    async fn handle_identify_block(handle: &GatewayHandle, ws: WebSocket, state: &mut GatewayState, reason: &str, cooldown: u64) -> Result<(), worker::Error> {
        state.ws_url = None;
        state.identify_cooldown_until = Some(cooldown as f64);
        DiscordGateway::save_state_from_handle(handle, state).await?;

        {
            let mut guard = handle.inner.reconnect_planned.lock().await;
            *guard = true;
        }
        
        let _ = ws.close(Some(4000), Some(reason)); // 4000 è il codice di riconnessione interna

        let mut guard = handle.inner.upstream.lock().await;
        *guard = None;

        let scheduled_date = js_sys::Date::new(&(js_sys::Date::now() + (cooldown as f64)).into());
        let scheduled_time = ScheduledTime::new(scheduled_date);

        let guard = handle.inner.storage.lock().await;
        guard.set_alarm(scheduled_time).await?;
        Ok(())
    }

    async fn stop_reconnecting(handle: &GatewayHandle, code: u16, reason: String) -> Result<(), worker::Error> {
        // 1. Carica lo stato corrente (o usa un default se non esiste)
        let mut state = DiscordGateway::load_state_from_handle(handle).await?.unwrap_or_default();
        let policy = classify_close_code(code);

        // 2. Aggiorna lo stato in memoria (inner)
        {
            let mut guard = handle.inner.reconnect_disabled.lock().await;
            *guard = true;
        }

        // 3. Aggiorna il modello di persistenza
        state.ws_url = None;
        state.heartbeat_interval_ms = None;
        state.reconnect_disabled = true;

        if !policy.can_resume {
            state.session_id = None;
            state.sequence = None;
            state.reconnect_strategy = ReconnectStrategy::IdentifyOnly;
        }

        DiscordGateway::save_state_from_handle(handle, &state).await?;

        let guard = handle.inner.storage.lock().await;
        guard.delete_alarm().await?;

        console_error!("discord-gateway: stopped reconnecting due to close code {code} and reason {reason}");

        Ok(())
    }

    async fn open_websocket(handle: &GatewayHandle, url: &str) -> Result<(), OpenWebSocketError> {
        let ws_url = format!("{}?v={}&encoding=json", to_http_url(url), GATEWAY_VERSION);

        let headers = Headers::new();
        headers.set("Upgrade", "websocket").ok();

        let mut init = RequestInit::new();
        init.with_method(Method::Get);
        init.with_headers(headers);

        let request = Request::new_with_init(&ws_url, &init)
            .map_err(|e| OpenWebSocketError { error: e.to_string(), retryable: true })?;

        let response = Fetch::Request(request).send().await
            .map_err(|e| OpenWebSocketError { error: e.to_string(), retryable: true })?;

        let maybe_error = OpenWebSocketError {
            error: format!("failed to connect ({})", response.status_code()),
            retryable: response.status_code() == 429 || response.status_code() >= 500 || response.status_code() == 0,
        };

        let ws = response.websocket().ok_or_else(|| maybe_error)?;

        ws.accept().map_err(|e| OpenWebSocketError { error: e.to_string(), retryable: true })?;

        {
            let mut guard = handle.inner.upstream.lock().await;
            *guard = Some(ws.clone());
            let mut guard = handle.inner.suppress_reconnect.lock().await;
            *guard = false;
            let mut guard = handle.inner.reconnect_planned.lock().await;
            *guard = false;
        }

        let state_guard = handle.inner.state.lock().await;
        
        let handle = handle.clone();


        state_guard.wait_until(async move {
            let mut events = ws.events()
                .map_err(|e| OpenWebSocketError { error: e.to_string(), retryable: true })
                .expect("Error opening websocket stream: ");

            loop {
                match events.try_next().await {
                    Ok(Some(WebsocketEvent::Message(msg))) => {
                        if let Some(text) = msg.text() {
                            if let Err(e) = DiscordGateway::handle_gateway_message(&handle, text.as_str()).await {
                                console_error!("discord-gateway: message handler error: {:?}", e);
                            }
                        }
                    }
                    Ok(Some(WebsocketEvent::Close(event))) => {
                        DiscordGateway::handle_websocket_close(&handle, event.code(), event.reason()).await;
                        break;
                    }
                    Ok(None) => continue,
                    Err(e) => {
                        DiscordGateway::handle_websocket_error(&handle, e).await;
                        break;
                    }
                }
            }

            ws.close(Some(0), Some("Something went wrong")).expect("Error closing websocket");

        });

        Ok(())
    }

    async fn get_gateway_info(bot_token: &str) -> Result<GatewayInfo, GatewayError> {
        let headers = Headers::new();
        headers.set("Authorization", &format!("Bot {}", bot_token))
            .map_err(|e| GatewayError::Network(e.to_string()))?;

        let request = Request::new_with_init(
            GATEWAY_BOT_URL,
            worker::RequestInit::new().with_method(Method::Get).with_headers(headers)
        ).map_err(|e| GatewayError::Network(e.to_string()))?;

        let mut response = Fetch::Request(request).send().await
            .map_err(|e| GatewayError::Network(e.to_string()))?;

        if response.status_code() != 200 {
            let status = response.status_code();
            let retryable = status == 429 || status >= 500 || status == 408;
            return Err(GatewayError::Api(
                format!("GET /gateway/bot failed ({})", status),
                retryable,
                status
            ));
        }

        let data: GatewayBotResponse = response.json().await
            .map_err(|e| GatewayError::Network(format!("Failed to parse JSON: {}", e)))?;

        if data.url.is_empty() {
            return Err(GatewayError::Api("GET /gateway/bot returned no URL".to_string(), true, 200));
        }

        Ok(GatewayInfo {
            url: data.url,
            session_start_limit: data.session_start_limit,
        })
    }

    async fn handle_gateway_message(handle: &GatewayHandle, message: &str) -> Result<()> {
        let deserializer = match GatewayEventDeserializer::from_json(message) {
            Some(d) => d,
            None => return Ok(())
        };

        let mut json_deserializer = serde_json::Deserializer::from_str(message);

        let event: GatewayEvent = match deserializer.deserialize(&mut json_deserializer) {
            Err(_) => return Ok(()),
            Ok(p) => p
        };

        // 2. Caricamento stato
        let mut state = DiscordGateway::load_state_from_handle(handle)
            .await?
            .unwrap_or_else(|| GatewayState::default());

        if let GatewayEvent::Dispatch(seq, _) = &event {
            state.sequence = Some(*seq);
            DiscordGateway::save_state_from_handle(handle, &state).await?;
        }

        match event {
            GatewayEvent::Hello(hello_payload) => DiscordGateway::handle_hello(handle, hello_payload, &mut state).await?,
            GatewayEvent::Dispatch(_seq, dispatch_event) => DiscordGateway::handle_dispatch(handle, dispatch_event, &mut state).await?,
            GatewayEvent::Heartbeat => DiscordGateway::send_heartbeat(handle, &state).await?,
            GatewayEvent::HeartbeatAck => DiscordGateway::handle_heartbeat_ack(handle, &mut state).await?,
            GatewayEvent::Reconnect => DiscordGateway::handle_reconnect(handle).await?,
            GatewayEvent::InvalidateSession(resumable) => DiscordGateway::handle_invalid_session(handle, resumable, &mut state).await?,
        };

        Ok(())
    }

    async fn handle_websocket_error(handle: &GatewayHandle, error: worker::Error) {
        console_error!("discord-gateway: WebSocket error: {:?}", error);

        // 1. Reset upstream e controllo logica di riconnessione
        let (should_reconnect, strategy) = {
            // Se il socket attivo è cambiato nel frattempo, usciamo
            // Nota: Qui dovresti passare l'identificatore del socket se ne hai più di uno,
            // ma nel tuo caso (DO single-socket), basta controllare se upstream è ancora quello
            let mut guard  = handle.inner.upstream.lock().await;
            *guard = None;

            let mut suppress_guard = handle.inner.suppress_reconnect.lock().await;
            let reconnect_guard = handle.inner.reconnect_planned.lock().await;

            if *suppress_guard {
                *suppress_guard = false;
                (false, None)
            } else if *reconnect_guard {
                (false, None)
            } else {
                (true, Some(ReconnectStrategy::ResumeOrIdentify))
            }
        }; // Il borrow finisce qui

        // 2. Esegui la riconnessione se necessario
        if should_reconnect {
            let options = ReconnectOptions {
                strategy: strategy,
                clear_session: false,
                reason: Some("websocket error event".to_string()),
            };
            
            let _ = DiscordGateway::reconnect_with_backoff(handle, Some(options)).await;
        }
    }

    async fn handle_websocket_close(handle: &GatewayHandle, code: u16, reason: String) {
        console_warn!("discord-gateway: WebSocket closed with code {code} and reason: {reason}");

        let action = {
            let mut guard_upstream = handle.inner.upstream.lock().await;
            *guard_upstream = None;
            
            let mut guard_suppress = handle.inner.suppress_reconnect.lock().await;
            let mut planned_guard = handle.inner.reconnect_planned.lock().await;
            let mut disabled_guard = handle.inner.reconnect_disabled.lock().await;

            if *guard_suppress {
                *guard_suppress = false;
                None // Nessuna azione
            } else if *planned_guard {
                *planned_guard = false;
                None // Nessuna azione
            } else {
                let policy = classify_close_code(code);
                if !policy.should_reconnect {
                    *disabled_guard = true;
                    Some(CloseAction::Stop(code, reason.clone()))
                } else {
                    Some(CloseAction::Reconnect(policy))
                }
            }
        };

        match action {
            Some(CloseAction::Stop(c, r)) => {
                if let Err(e) = DiscordGateway::stop_reconnecting(handle, c, r).await {
                    console_error!("discord-gateway: stopReconnecting failed: {:?}", e);
                }
            }
            Some(CloseAction::Reconnect(policy)) => {
                let options = ReconnectOptions {
                    strategy: Some(if policy.can_resume { 
                        ReconnectStrategy::ResumeOrIdentify 
                    } else { 
                        ReconnectStrategy::IdentifyOnly 
                    }),
                    clear_session: !policy.can_resume,
                    reason: Some(format!("close {}: {}", code, reason)),
                };
                let _ = DiscordGateway::reconnect_with_backoff(handle, Some(options)).await;
            }
            None => {}
        }
    }

    async fn handle_hello(handle: &GatewayHandle, hello: Hello, state: &mut GatewayState) -> Result<(), worker::Error> {
        // 1. Aggiorna lo stato
        state.heartbeat_interval_ms = Some(hello.heartbeat_interval);
        state.last_heartbeat_ack = Some(js_sys::Date::now());
        DiscordGateway::save_state_from_handle(handle, state).await?;

        DiscordGateway::identify_or_resume(handle, state).await?;

        // 3. Setup del primo heartbeat (jitter)
        // Math::random() in JS corrisponde a rand::random() o js_sys::Math::random()
        let jitter = js_sys::Math::random() * (hello.heartbeat_interval as f64);
        let first_delay = jitter as u64;
        
        // Impostiamo l'allarme (usando la API di Cloudflare per i Durable Objects)
        // NOTA: set_alarm accetta un timestamp in millisecondi
        let alarm_time = js_sys::Date::now() as u64 + first_delay;
        let scheduled_time = ScheduledTime::new(js_sys::Date::new(&alarm_time.into()));

        let guard = handle.inner.storage.lock().await;
        guard.set_alarm(scheduled_time).await?;

        Ok(())
    }

    async fn handle_heartbeat_ack(handle: &GatewayHandle, state: &mut GatewayState) -> Result<(), worker::Error> {
        state.last_heartbeat_ack = Some(js_sys::Date::now());
        DiscordGateway::save_state_from_handle(handle, state).await?;
        
        Ok(())
    }

    async fn handle_dispatch(handle: &GatewayHandle, event: DispatchEvent, state: &mut GatewayState) -> Result<(), worker::Error> {
        match event {
            DispatchEvent::Ready(ready) => {
                state.session_id = Some(ready.session_id.clone());
                state.resume_gateway_url = Some(ready.resume_gateway_url.clone());
                state.reconnect_attempts = 0;

                DiscordGateway::save_state_from_handle(handle, state).await?;
                console_log!("discord-gateway: READY (Session: {})", ready.session_id);
            }

            DispatchEvent::Resumed => {
                state.reconnect_attempts = 0;
                DiscordGateway::save_state_from_handle(handle, state).await?;
                console_log!("discord-gateway: RESUMED");
            }

            // Inoltra tutti gli altri eventi che il tuo bot deve gestire
            other => {
                if is_forwarded_event_type(other.kind()) {
                    let data = serde_json::to_value(&other)?;

                    DiscordGateway::forward_event(handle, other.kind(), data).await?;
                }
            }
        }
        Ok(())
    }

    async fn handle_reconnect(handle: &GatewayHandle) -> Result<(), worker::Error> {
        console_warn!("discord-gateway: server requested reconnect");

        // Controlliamo il rate limit usando i timestamp salvati nello stato
        if DiscordGateway::is_reconnect_rate_limited(handle).await {
            console_warn!("discord-gateway: reconnect rate limited — falling back to backoff");

            let options = ReconnectOptions {
                strategy: Some(ReconnectStrategy::ResumeOrIdentify),
                clear_session: false, 
                reason: Some("reconnect rate limited".into())
            };

            DiscordGateway::reconnect_with_backoff(handle, Some(options)).await?;
            return Ok(());
        }

        // Aggiungiamo il timestamp attuale ai tentativi di riconnessione
        let now = js_sys::Date::now();
        {
            let mut timestamps_guard = handle.inner.reconnect_timestamps.lock().await;
            timestamps_guard.push_back(now);
        }

        let options = ReconnectOptions { 
            strategy: Some(ReconnectStrategy::ResumeOrIdentify), 
            clear_session: false, 
            reason: Some("reconnect request by discord".into())
        };

        // Riconnessione immediata (con un ritardo minimo di sicurezza)
        DiscordGateway::reconnect_with_min_delay(handle, Some(options)).await?;
        
        Ok(())
    }

    async fn handle_invalid_session(handle: &GatewayHandle, resumable: bool, state: &mut GatewayState) -> Result<(), worker::Error> {    
        console_warn!("discord-gateway: invalid session, resumable: {}", resumable);

        // 1. Aggiornamento strategia e reset stato
        state.reconnect_strategy = if resumable {
            ReconnectStrategy::ResumeOrIdentify
        } else {
            ReconnectStrategy::IdentifyOnly
        };

        if !resumable {
            state.session_id = None;
            state.sequence = None;
        }

        state.ws_url = None;
        state.heartbeat_interval_ms = None;
        DiscordGateway::save_state_from_handle(handle, state).await?;

        // 2. Chiusura del socket corrente
        {
            let mut upstream_guard = handle.inner.upstream.lock().await;
            if let Some(ws) = upstream_guard.take() {
                let mut planned_guard = handle.inner.reconnect_planned.lock().await;
                *planned_guard = true;
                // Chiudiamo il WebSocket. In workers-rs il metodo close richiede 
                // codice di chiusura e ragione.
                let _ = ws.close(Some(4000), Some("invalid session"));
            }
        }

        // 3. Setup del delay (Discord richiede 1-5s)
        let delay = 1000.0 + js_sys::Math::random() * 4000.0;
        let alarm_time = js_sys::Date::now() as u64 + delay as u64;

        let scheduled_time = ScheduledTime::new(js_sys::Date::new(&alarm_time.into()));

        let guard = handle.inner.storage.lock().await;
        guard.set_alarm(scheduled_time).await?;

        Ok(())
    }

    async fn schedule_heartbeat(handle: &GatewayHandle, state: &GatewayState) -> Result<(), worker::Error> {
        // Se l'intervallo è None, non facciamo nulla
        if let Some(interval) = state.heartbeat_interval_ms {
            // Calcoliamo il timestamp futuro. 
            // In Wasm/Workers, js_sys::Date::now() restituisce un f64.
            let next_beat = js_sys::Date::new(&((js_sys::Date::now() as u64) + interval).into());
            let scheduled_time = ScheduledTime::new(next_beat);
            
            // Impostiamo l'allarme tramite lo storage
            let guard = handle.inner.storage.lock().await;
            guard.set_alarm(scheduled_time).await?;
        }
        Ok(())
    }

    async fn send_heartbeat(handle: &GatewayHandle, state: &GatewayState) -> Result<(), worker::Error> {
        let ws = {
            let guard = handle.inner.upstream.lock().await;
            guard.clone()
        };

        if let Some(ws) = ws {
            let heartbeat = Heartbeat::new(state.sequence);
            ws.send(&heartbeat)
                .map_err(|e| worker::Error::from(e.to_string()))?;
        }
        Ok(())
    }

    async fn forward_event(handle: &GatewayHandle, event_type: EventType, data: serde_json::Value) -> Result<(), worker::Error> { 
        let creds = match DiscordGateway::load_credentials_from_handle(handle).await? {
            Some(c) => c,
            None => return Ok(()),
        };

        let Some(webhook_url) = creds.webhook_url else {
            return Ok(());
        };

        let mut event = json!({
            "event_type": format!("GATEWAY_{:?}", event_type.name()),
            "timestamp": js_sys::Date::now() as u64
        });

        if let Some(map) = event.as_object_mut() {
            if let Some(data_obj) = data.as_object() {
                for (k, v) in data_obj {
                    map.insert(k.clone(), v.clone());
                }
            }
        }

        let body = serde_json::to_string(&event)
            .map_err(|e| worker::Error::from(format!("JSON serialization error: {}", e)))?;

        for attempt in 0..WEBHOOK_MAX_ATTEMPTS {
            let headers = Headers::new();
            headers.set("Content-Type", "application/json")?;
            headers.set(
                "x-discord-gateway-token",
                creds.webhook_secret.as_deref().unwrap_or(&creds.bot_token),
            )?;

            let request = Request::new_with_init(
                &webhook_url,
                RequestInit::new()
                    .with_method(Method::Post)
                    .with_headers(headers)
                    .with_body(Some(body.clone().into())),
            )?;

            match Fetch::Request(request).send().await {
                Ok(response) if response.status_code() >= 200 && response.status_code() < 300 => {
                    return Ok(()); // Successo
                }
                Ok(mut response) => {
                    let status = response.status_code();
                    let error_text = response.text().await.unwrap_or_default();
                    
                    console_error!(
                        "discord-gateway: webhook forward failed (attempt {}): status {}, error: {}",
                        attempt + 1, status, error_text
                    );

                    // Se 4xx, non riprovare (client error)
                    if status >= 400 && status < 500 {
                        return Ok(());
                    }
                }
                Err(e) => {
                    console_error!(
                        "discord-gateway: webhook forward network error (attempt {}): {}",
                        attempt + 1, e
                    );
                }
            }

            // Attesa prima del retry
            if attempt < WEBHOOK_MAX_ATTEMPTS - 1 {
                // Qui bisogna aspettare per: WEBHOOK_RETRY_DELAY_MS
            }
        }

        Ok(())
    }

    // State

    async fn save_state(&self, state: &GatewayState) -> Result<(), worker::Error> {
        let guard = self.inner.storage.lock().await;
        guard.put(STATE_KEY, state).await
    }

    async fn save_state_from_handle(handle: &GatewayHandle, state: &GatewayState) -> Result<(), worker::Error> {
        let guard = handle.inner.storage.lock().await;
        guard.put(STATE_KEY, state).await
    }

    async fn load_state(&self) -> Result<Option<GatewayState>, worker::Error> {
        let guard = self.inner.storage.lock().await;
        let state = guard.get::<GatewayState>(STATE_KEY).await?;
        Ok(state)
    }

    async fn load_state_from_handle(handle: &GatewayHandle) -> Result<Option<GatewayState>, worker::Error> {
        let guard = handle.inner.storage.lock().await;
        let state = guard.get::<GatewayState>(STATE_KEY).await?;
        Ok(state)
    }

    #[allow(unused)]
    async fn load_credentials(&self) -> Result<Option<GatewayCredentials>, worker::Error> {
        // 1. Controlla la cache nel RefCell
        let mut guard = self.inner.cached_credentials.lock().await;
        if let Some(ref creds) = *guard {
            return Ok(Some(creds.clone()));
        }

        let mut storage_guard = self.inner.storage.lock().await;
        let creds = storage_guard.get::<GatewayCredentials>(CREDENTIALS_KEY).await?;

        // 3. Popola la cache
        if let Some(ref c) = creds {
            *guard = Some(c.clone());
        }

        Ok(creds)
    }

    async fn load_credentials_from_handle(handle: &GatewayHandle) -> Result<Option<GatewayCredentials>, worker::Error> {
        // 1. Controlla la cache nel RefCell
        let mut guard = handle.inner.cached_credentials.lock().await;
        if let Some(ref creds) = *guard {
            return Ok(Some(creds.clone()));
        }

        // 2. Carica dallo storage se la cache è vuota
        let storage_guard = handle.inner.storage.lock().await;
        let creds = storage_guard.get::<GatewayCredentials>(CREDENTIALS_KEY).await?;

        // 3. Popola la cache
        if let Some(ref c) = creds {
            *guard = Some(c.clone());
        }

        Ok(creds)
    }

    fn as_handle(&self) -> GatewayHandle {
        GatewayHandle::from(self)
    }
}