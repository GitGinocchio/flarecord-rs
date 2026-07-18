use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Connected,
    Connecting,
    Disconnected
}

#[derive(Serialize)]
pub struct GatewayStatus {
    pub status: Status,
    pub session_id: Option<String>,
    pub connected_at: Option<f64>,
    pub sequence: Option<u64>,
    pub reconnect_attempts: u32,
}