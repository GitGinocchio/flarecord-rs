use std::sync::Arc;

use futures::lock::Mutex;
use worker::Storage;

use crate::gateway::{DiscordGateway, inner::GatewayInner};

pub struct GatewayHandle {
    pub inner: Arc<Mutex<GatewayInner>>,
    pub storage: Arc<Storage>,
}

impl GatewayHandle {
    pub fn from_gateway(gateway: &DiscordGateway) -> Self {
        Self {
            inner: gateway.inner.clone(),
            storage: Arc::new(gateway.state.storage())
        }
    }
}

impl Clone for GatewayHandle {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            storage: self.storage.clone()
        }
    }
}