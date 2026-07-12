use std::collections::HashMap;
use std::sync::{Arc, LazyLock, OnceLock};

use reqwest::{Client};
use twilight_model::application::interaction::Interaction as TwilightInteraction;
use worker::{Env, Request, Response};

use crate::bot::builder::BotBuilder;
use crate::crypto;
use crate::models::command::Command;
use crate::models::components::ComponentType;
use crate::models::interaction::Interaction;
use crate::models::modals::ModalType;
use crate::error::Error;
use crate::utils::{has_api_access, is_interaction};

pub mod builder;
pub mod state;

pub (crate) static HTTP_CLIENT: LazyLock<Client> = LazyLock::new(|| Client::new());
static BOT: OnceLock<Arc<Bot>> = OnceLock::new();

#[allow(unused)]
pub struct Bot {
    pub (crate) commands: HashMap<String, Arc<dyn Command>>,
    pub (crate) components: HashMap<String, ComponentType>,
    pub (crate) modals: HashMap<String, ModalType>
}

impl Bot {
    pub (crate) fn set_global(self) {
        let bot = Arc::new(self);
        BOT.set(bot)
            .map_err(|_| ())
            .expect("Bot already initialized")
    }

    pub (crate) fn get_global() -> Arc<Bot> {
        BOT.get().expect("Bot not initiliazed").clone()
    }

    pub fn new() -> Arc<Bot> {
        BotBuilder::new().build()
    }

    pub async fn handle_commands(&self, mut req: Request, env: Env) -> worker::Result<Response> {
        let body = req.bytes().await?;
        let headers = req.headers();

        let public_key = env.secret("DISCORD_BOT_PUBLIC_KEY")
            .map_err(|e| Error::EnvironmentVariableNotFound(format!("{e}")))?
            .to_string();
    
        let is_valid = match crypto::verify_signature(headers, &body, &public_key) {
            Err(e) => return e.as_response(),
            Ok(value) => value
        };

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

    pub async fn handle_api(&self, req: Request, env: Env) -> worker::Result<Response> {
        let headers = req.headers();
        let token = env.secret("DISCORD_BOT_TOKEN")
            .map_err(|e| Error::EnvironmentVariableNotFound(format!("{e}")))?
            .to_string();

        if !has_api_access(headers, &token) {
            return Response::error("Unauthorized", 401);
        }

        let endpoint = req.path();
        let segments: Vec<&str> = endpoint.split('/').filter(|s| !s.is_empty()).collect();

        match segments.as_slice() {
            [.., "sync"] => crate::api::sync::sync(env, token).await,
            _ => Response::error("Not found", 404)
        }
    }

    pub async fn handle(&self, req: Request, env: Env) -> worker::Result<Response> {
        let headers = req.headers();
       
        if is_interaction(headers) {
            return self.handle_commands(req, env).await
        }

        self.handle_api(req, env).await
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