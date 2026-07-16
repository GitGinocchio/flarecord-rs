use std::sync::Arc;

use crate::{bot::Bot, services::discord::{DISCORD_SERVICE, DiscordService}};

#[allow(unused)]
pub struct BotState {
    bot: Arc<Bot>,
    pub discord: Arc<DiscordService>
}

impl BotState {
    pub fn new(bot: Arc<Bot>) -> Self {
        let discord_service = DISCORD_SERVICE.get().expect("Expected global discord service to be Some");
        Self { 
            bot,
            discord: discord_service.clone()
        }
    }
}