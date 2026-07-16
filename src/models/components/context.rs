use std::sync::Arc;
use worker::Env;

use crate::{
    bot::state::BotState, services::discord::DiscordService
};

pub struct ComponentContext {
    pub bot: BotState,
    pub env: Env,
    pub discord: Arc<DiscordService>
}

impl ComponentContext {
    pub fn new(bot: BotState, env: Env, discord: Arc<DiscordService>) -> Self {
        Self {
            bot: bot, 
            env: env,
            discord: discord
        }
    }
}