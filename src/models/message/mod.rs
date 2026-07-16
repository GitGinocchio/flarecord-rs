use serde::Serialize;
use serde_json::Value;



// TODO: Spostare questo in un altro file
#[derive(Serialize, Default, Debug)]
pub struct DiscordMessagePayload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub embeds: Option<Vec<Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Vec<Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<bool>,
}