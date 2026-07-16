pub mod button;
pub mod select;

use std::pin::Pin;
use std::future::Future;

use crate::error::BotResult;
use crate::models::command::response::CommandResponse;
use crate::models::components::context::ComponentContext;
use crate::models::components::interaction::ComponentInteraction;

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;

pub trait InteractiveComponentHandler: Send + Sync + 'static {
    fn handle(&self, int: ComponentInteraction, ctx: ComponentContext) -> BoxFuture<'static, BotResult<()>>;
}

impl<F, Fut> InteractiveComponentHandler for F
where
    F: Fn(ComponentInteraction, ComponentContext) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = BotResult<()>> + 'static,
{
    fn handle(&self, int: ComponentInteraction, ctx: ComponentContext) -> BoxFuture<'static, BotResult<()>> {
        Box::pin(self(int, ctx))
    }
}

pub struct Handler<F>(pub F);

impl<F, Fut> InteractiveComponentHandler for Handler<F>
where
    F: Fn(ComponentInteraction, ComponentContext) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = BotResult<()>> + 'static,
{
    fn handle(&self, int: ComponentInteraction, ctx: ComponentContext) -> BoxFuture<'static, BotResult<()>> {
        Box::pin(self.0(int, ctx))
    }
}