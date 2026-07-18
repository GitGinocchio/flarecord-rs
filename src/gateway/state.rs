use serde::{Deserialize, Serialize};

use crate::gateway::utils::ReconnectStrategy;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GatewayState {
    pub ws_url: Option<String>,
    pub resume_gateway_url: Option<String>,
    pub session_id: Option<String>,
    pub sequence: Option<u64>,
    pub heartbeat_interval_ms: Option<u64>,
    pub last_heartbeat_ack: Option<f64>,
    pub connected_at: Option<f64>,
    pub reconnect_attempts: u32,
    pub reconnect_strategy: ReconnectStrategy,
    pub identify_cooldown_until: Option<f64>,
    pub session_start_remaining: Option<u32>,
    pub session_start_reset_after_ms: Option<u64>,
    pub session_start_total: Option<u32>,
    pub session_start_max_concurrency: Option<u32>,
    pub reconnect_disabled: bool,
}

impl Default for GatewayState {
    fn default() -> Self {
        Self {
            ws_url: None,
            resume_gateway_url: None,
            session_id: None,
            sequence: None,
            heartbeat_interval_ms: None,
            last_heartbeat_ack: None,
            connected_at: None,
            reconnect_attempts: 0,
            reconnect_strategy: ReconnectStrategy::ResumeOrIdentify,
            identify_cooldown_until: None,
            session_start_remaining: None,
            session_start_reset_after_ms: None,
            session_start_total: None,
            session_start_max_concurrency: None,
            reconnect_disabled: false,
        }
    }
}