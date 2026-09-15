use std::sync::Arc;
use worker::Env;

use crate::{
    bot::state::BotState, error::BotResult, services::{discord::DiscordService}
};



pub struct InteractionContext {
    pub bot: BotState,
    pub env: Env,
    pub discord: Arc<DiscordService>,
}

impl InteractionContext {
    pub fn new(bot: BotState, env: Env) -> BotResult<Self> {
        Ok(Self {
            bot: bot, 
            env: env,
            discord: DiscordService::get()?,
        })
    }
}