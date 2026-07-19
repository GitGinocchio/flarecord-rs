use std::{collections::VecDeque, sync::Arc};

use futures::lock::Mutex;
use worker::{Env, State, Storage};

use crate::gateway::credentials::GatewayCredentials;


pub struct GatewayInner {
    pub upstream: Arc<Mutex<Option<worker::web_sys::WebSocket>>>,

    pub state: Arc<Mutex<State>>,
    pub storage: Arc<Mutex<Storage>>,
    pub env: Env,

    pub suppress_reconnect: Arc<Mutex<bool>>,
    pub reconnect_planned: Arc<Mutex<bool>>,
    pub reconnect_disabled: Arc<Mutex<bool>>,
    
    pub reconnect_timestamps: Arc<Mutex<VecDeque<f64>>>,
    
    pub cached_credentials: Arc<Mutex<Option<GatewayCredentials>>>,
}

impl GatewayInner {
    pub fn new(env: Env, state: State) -> Self {
        Self {
            env: env,
            storage: Arc::new(Mutex::new(state.storage())),
            state: Arc::new(Mutex::new(state)),
            upstream: Arc::new(Mutex::new(None)),
            suppress_reconnect: Arc::new(Mutex::new(false)),
            reconnect_planned: Arc::new(Mutex::new(false)),
            reconnect_disabled: Arc::new(Mutex::new(false)),
            reconnect_timestamps: Arc::new(Mutex::new(VecDeque::new())),
            cached_credentials: Arc::new(Mutex::new(None))
        }
    }
}