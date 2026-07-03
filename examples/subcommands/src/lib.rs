use std::sync::{Arc, LazyLock};

use worker::*;

use flarecord::bot::{Bot, builder::BotBuilder};

mod commands;
use commands::mycommand::MyCommand;

static BOT: LazyLock<Arc<Bot>> = LazyLock::new(|| {
    BotBuilder::new()
        .enable_dev_commands()
        .register_command(MyCommand)
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