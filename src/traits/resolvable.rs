use crate::error::BotResult;

pub trait Resolvable<T> {
    async fn resolve(&self) -> BotResult<T>;
}