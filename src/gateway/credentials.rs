use serde::{Deserialize, Serialize};




#[derive(Serialize, Deserialize, Clone)]
pub struct GatewayCredentials {
    pub bot_token: String,
    #[serde(default)]
    pub webhook_url: Option<String>,
    #[serde(default)]
    pub webhook_secret: Option<String>
}