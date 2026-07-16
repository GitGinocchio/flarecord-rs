use std::sync::{Arc, LazyLock};

use worker::*;

use flarecord::bot::{Bot, builder::BotBuilder};

mod commands;

static BOT: LazyLock<Arc<Bot>> = LazyLock::new(|| {
    BotBuilder::new()
        .enable_bot_commands()
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