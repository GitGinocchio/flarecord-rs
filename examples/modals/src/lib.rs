use std::sync::{Arc, LazyLock};

use worker::*;

use flarecord::bot::{Bot, builder::BotBuilder};

pub mod commands;
pub mod modals;

static BOT: LazyLock<Arc<Bot>> = LazyLock::new(|| {
    BotBuilder::new()
        .enable_default_commands()
        .register_command(commands::ticket::Ticket)
        .build()
});

#[event(fetch)]
async fn fetch(
    req: Request,
    env: Env,
    _ctx: Context,
) -> Result<Response> {
    BOT.handle(req, env).await
}