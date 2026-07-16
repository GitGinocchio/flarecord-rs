use crate::error::BotResult;

#[allow(async_fn_in_trait)]
pub trait Resolvable<T> {
    async fn resolve(&self) -> BotResult<T>;
}