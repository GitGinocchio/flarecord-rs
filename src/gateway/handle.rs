use std::sync::Arc;

use crate::gateway::{DiscordGateway, inner::GatewayInner};

pub struct GatewayHandle {
    pub inner: Arc<GatewayInner>,
}

impl From<&DiscordGateway> for GatewayHandle {
    fn from(value: &DiscordGateway) -> Self {
        Self {
            inner: value.inner.clone()
        }
    }
}

impl Clone for GatewayHandle {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone()
        }
    }
}