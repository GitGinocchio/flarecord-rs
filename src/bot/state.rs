use std::sync::Arc;

use crate::bot::Bot;

#[allow(unused)]
pub struct BotState {
    bot: Arc<Bot>
}

impl BotState {
    pub fn new(bot: Arc<Bot>) -> Self {
        Self { bot }
    }
}