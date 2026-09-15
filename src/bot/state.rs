use std::{sync::Arc};

use worker::{Env, Request, Response};

use crate::{bot::Bot, error::BotResult};

#[allow(unused)]
pub struct BotState {
    bot: Arc<Bot>
}

impl BotState {
    pub fn new(bot: Arc<Bot>) -> Self {
        Self { 
            bot
        }
    }

    pub fn get_commands(&self) -> Vec<&String> {
        self.bot.commands.keys().collect()
    }

    pub fn get_components(&self) -> Vec<&String> {
        self.bot.components.keys().collect()
    }

    pub fn get_modals(&self) -> Vec<&String> {
        self.bot.modals.keys().collect()
    }

    pub async fn send_api_request(&self, req: Request, env: Env) -> BotResult<Response> {
        self.bot.handle_api(req, env)
            .await
            .map_err(|e| e.into())
    }
}