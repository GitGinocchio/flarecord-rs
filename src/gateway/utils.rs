use serde::{Deserialize, Serialize};

use crate::gateway::{constants::{non_reconnectable_close_codes, non_resumable_close_codes}, state::GatewayState};

#[derive(Deserialize, Clone, Debug)]
pub struct SessionStartLimit {
    pub total: u32,
    pub remaining: u32,
    pub reset_after: u64,
    pub max_concurrency: u32,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct GatewayBotResponse {
    pub url: String,
    pub shards: u32,
    pub session_start_limit: SessionStartLimit,
}

#[derive(Deserialize, Clone, Debug)]
pub struct GatewayInfo {
    pub url: String,
    pub session_start_limit: SessionStartLimit
}

pub enum GatewayError {
    Api(String, bool, u16), // Message, Retryable, Status
    Network(String),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ReconnectStrategy {
    ResumeOrIdentify,
    IdentifyOnly
}

pub struct CodeCloseClassification {
    pub should_reconnect: bool,
    pub can_resume: bool
}

pub enum CloseAction {
    Stop(u16, String),
    Reconnect(CodeCloseClassification),
}

pub struct ReconnectOptions {
    pub strategy: Option<ReconnectStrategy>,
    pub clear_session: bool,
    pub reason: Option<String>,
}

#[derive(Debug)]
pub struct ConnectError {
    pub error: String, 
    pub retry_scheduled: bool
}

#[derive(Debug)]
pub struct OpenWebSocketError {
    pub error: String,
    pub retryable: bool
}

impl From<worker::Error> for ConnectError {
    fn from(value: worker::Error) -> Self {
        Self {
            error: value.to_string(),
            retry_scheduled: false
        }
    }
}

impl From<String> for ConnectError {
    fn from(value: String) -> Self {
        Self {
            error: value,
            retry_scheduled: false
        }
    }
}

impl From<ConnectError> for worker::Error {
    fn from(value: ConnectError) -> worker::Error {
        worker::Error::from(format!("ConnectError(error={}, retry_scheduled={})", value.error, value.retry_scheduled))
    }
}

pub fn can_resume(state: &GatewayState) -> bool {
    state.reconnect_strategy != ReconnectStrategy::IdentifyOnly
        && state.session_id.is_some() 
        && state.sequence.is_some()
}

pub fn classify_close_code(code: u16) -> CodeCloseClassification {
    let should_reconnect = !non_reconnectable_close_codes().contains(&code);
    let can_resume = should_reconnect && !non_resumable_close_codes().contains(&code);
    CodeCloseClassification { should_reconnect, can_resume }
}

pub fn is_private_hostname(hostname: &str) -> bool {
    let lower = hostname.to_lowercase();
    if lower == "localhost" || lower.ends_with(".localhost") || lower.ends_with(".local") {
        return true;
    }
    is_private_ipv4(&lower) || is_private_ipv6(&lower)
}

pub fn is_private_ipv4(host: &str) -> bool {
    let parts: Vec<&str> = host.split('.').collect();
    if parts.len() != 4 { return false; }
    
    let nums: Vec<u8> = parts.iter().filter_map(|p| p.parse().ok()).collect();
    if nums.len() != 4 { return false; }

    let (a, b) = (nums[0], nums[1]);
    
    a == 10 || a == 127 || a == 0 || 
    (a == 169 && b == 254) || 
    (a == 172 && (16..=31).contains(&b)) || 
    (a == 192 && b == 168)
}

pub fn is_private_ipv6(host: &str) -> bool {
    if host.contains('.') {
        let last_part = host.split(':').last().unwrap_or("");
        if is_private_ipv4(last_part) { return true; }
    }
    
    let lower = host.to_lowercase();
    lower == "::1" || lower == "::" || 
    lower.starts_with(|c| matches!(c, 'f')) && (lower.starts_with("fc") || lower.starts_with("fd")) ||
    lower.starts_with("fe8") || lower.starts_with("fe9") || lower.starts_with("fea") || lower.starts_with("feb")
}

pub fn to_http_url(url: &str) -> String {
    if let Some(stripped) = url.strip_prefix("wss://") {
        format!("https://{}", stripped)
    } else if let Some(stripped) = url.strip_prefix("ws://") {
        format!("http://{}", stripped)
    } else {
        url.to_string()
    }
}