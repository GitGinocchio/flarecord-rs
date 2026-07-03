use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, OnceLock};

use reqwest::{Client};
use reqwest::header::{HeaderMap, HeaderValue};
use twilight_model::application::interaction::Interaction as TwilightInteraction;
use twilight_model::id::Id;
use twilight_model::id::marker::ApplicationMarker;
use worker::{Env, Request, Response};

use crate::bot::builder::BotBuilder;
use crate::models::command::Command;
use crate::models::components::ComponentType;
use crate::models::interaction::Interaction;
use crate::models::modals::ModalType;
use crate::error::Error;
use crate::crypto;
use crate::services::discord::DiscordService;

pub mod builder;
pub mod state;

pub (crate) static HTTP_CLIENT: OnceLock<Arc<Client>> = OnceLock::new();
static BOT: OnceLock<Arc<Bot>> = OnceLock::new();
static IS_INITIALIZED: AtomicBool = AtomicBool::new(false);

#[allow(unused)]
pub struct Bot {
    pub (crate) commands: HashMap<String, Arc<dyn Command>>,
    pub (crate) components: HashMap<String, ComponentType>,
    pub (crate) modals: HashMap<String, ModalType>
}

#[allow(unused)]
impl Bot {
    pub (crate) fn set_global(self) {
        let bot = Arc::new(self);
        BOT.set(bot).map_err(|_| worker::console_debug!("Bot already initialized"));
    }

    pub (crate) fn get_global() -> Arc<Bot> {
        BOT.get().expect("Bot not initiliazed").clone()
    }

    pub (crate) fn ensure_global_client(&self, token: &str) -> &Arc<Client> {
        if let Some(client) = HTTP_CLIENT.get() {
            return client
        }

        let mut headers = HeaderMap::new();
        headers.insert("Authorization", HeaderValue::from_str(&format!("Bot {}", token)).expect("Error parsing header value"));
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .expect("Error building reqwest::Client");

        HTTP_CLIENT.set(Arc::new(client));
        HTTP_CLIENT.get().expect("Client to be set")
    }

    pub (crate) fn new() -> Arc<Bot> {
        let bot = Self {
            commands: HashMap::new(),
            components: HashMap::new(),
            modals: HashMap::new()
        };
        bot.set_global();
        Bot::get_global()
    }

    #[deprecated]
    pub (crate) async fn sync_commands_once(&self, env: &Env) -> worker::Result<bool> {
        if IS_INITIALIZED.load(Ordering::Relaxed) {
            worker::console_debug!("Command synchronization not necessary");
            return Ok(true);
        }

        worker::console_debug!("Launching command synchronization");

        let application_id = env.secret("DISCORD_BOT_APPLICATION_ID")
            .map_err(|e| Error::EnvironmentVariableNotFound(format!("{e}")))?
            .to_string();

        let token = env.secret("DISCORD_BOT_TOKEN")
            .map_err(|e| Error::EnvironmentVariableNotFound(format!("{e}")))?
            .to_string();

        let client = self.ensure_global_client(&token);

        let discord_service = DiscordService::get_or_init(client.clone());
        //discord_service.update_global_commands().await?;

        IS_INITIALIZED.store(true, Ordering::Relaxed);

        Ok(false)
    }

    pub async fn handle(&self, mut req: Request, env: Env) -> worker::Result<Response> {
        let body = req.bytes().await?;
        let headers = req.headers();

        let public_key = env.secret("DISCORD_BOT_PUBLIC_KEY")
            .map_err(|e| Error::EnvironmentVariableNotFound(format!("{e}")))?
            .to_string();

        let token = env.secret("DISCORD_BOT_TOKEN")
            .map_err(|e| Error::EnvironmentVariableNotFound(format!("{e}")))?
            .to_string();

        self.ensure_global_client(&token);
    
        let is_valid = crypto::verify_signature(headers, &body, &public_key)?;

        if !is_valid {
            return Response::error("Unauthorized", 401);
        }

        let tw_interaction: TwilightInteraction = serde_json::from_slice(&body)?;
        let interaction = Interaction::from(tw_interaction);

        match interaction.perform(env).await {
            Ok(response) => Ok(response),
            Err(e) => {
                worker::console_debug!("Handler error: {e:?}");
                e.as_response()
            }
        }
    }
}

impl From<BotBuilder> for Bot {
    fn from(builder: BotBuilder) -> Self {
        Self {
            commands: builder.commands,
            components: builder.components,
            modals: builder.modals
        }
    }
}