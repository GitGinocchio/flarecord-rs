use std::{collections::VecDeque, sync::{Arc, Mutex}};

use worker::WebSocket;

use crate::gateway::credentials::GatewayCredentials;

pub struct GatewayInner {
    pub upstream: Option<WebSocket>,

    pub suppress_reconnect: bool,
    pub reconnect_planned: bool,
    pub reconnect_disabled: bool,
    
    pub reconnect_timestamps: VecDeque<f64>,
    
    pub cached_credentials: Option<GatewayCredentials>,

    // Capire a cosa serviva questo
    #[allow(unused)]
    pub processor_lock: Arc<Mutex<()>>,
}

impl Default for GatewayInner {
    fn default() -> Self {
        Self {
            upstream: None,
            suppress_reconnect: false,
            reconnect_planned: false,
            reconnect_disabled: false,
            reconnect_timestamps: VecDeque::new(),
            cached_credentials: None,
            processor_lock: Arc::new(Mutex::new(()))
        }
    }
}