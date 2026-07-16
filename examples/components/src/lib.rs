use std::sync::{Arc, LazyLock};

use worker::*;

use flarecord::bot::{Bot, builder::BotBuilder};

mod components;

mod commands;
use commands::hello::Hello;

use crate::components::mycomponent::MyComponent;

static BOT: LazyLock<Arc<Bot>> = LazyLock::new(|| {
    BotBuilder::new()
        .enable_bot_commands()
        .register_component(MyComponent)
        .register_command(Hello)
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