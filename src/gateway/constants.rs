use std::collections::HashSet;

use twilight_model::gateway::event::EventType;

// Costanti di base
pub const STATE_KEY: &str = "gateway_state";
pub const CREDENTIALS_KEY: &str = "credentials";
pub const GATEWAY_VERSION: u8 = 10;
pub const GATEWAY_BOT_URL: &str = "https://discord.com/api/v10/gateway/bot";

// Tempi e limiti (in millisecondi)
pub const MAX_BACKOFF_MS: u64 = 300_000;
pub const RECONNECT_RATE_LIMIT: u32 = 5;
pub const RECONNECT_RATE_WINDOW_MS: u64 = 60_000;
pub const ALARM_FALLBACK_DELAY_MS: u64 = 30_000;
pub const WEBHOOK_MAX_ATTEMPTS: u8 = 2;
pub const WEBHOOK_RETRY_DELAY_MS: u64 = 1_000;

// Codici di chiusura
pub const INTERNAL_RECONNECT_CLOSE_CODE: u16 = 3001;

pub fn non_reconnectable_close_codes() -> HashSet<u16> {
    HashSet::from([4004, 4010, 4011, 4012, 4013, 4014])
}

pub fn non_resumable_close_codes() -> HashSet<u16> {
    HashSet::from([4003, 4007, 4009])
}

pub fn is_forwarded_event_type(event_type: EventType) -> bool {
    matches!(event_type, EventType::MessageCreate | EventType::ReactionAdd | EventType::ReactionRemove);
    false
}

// Intents (usando i bit shift come in TS)
pub const INTENT_GUILDS: u32 = 1 << 0;
pub const INTENT_GUILD_MESSAGES: u32 = 1 << 9;
pub const INTENT_GUILD_MESSAGE_REACTIONS: u32 = 1 << 10;
pub const INTENT_DIRECT_MESSAGES: u32 = 1 << 12;
pub const INTENT_DIRECT_MESSAGE_REACTIONS: u32 = 1 << 13;
pub const INTENT_MESSAGE_CONTENT: u32 = 1 << 15;

pub const GATEWAY_INTENTS: u32 = INTENT_GUILDS 
    | INTENT_GUILD_MESSAGES 
    | INTENT_GUILD_MESSAGE_REACTIONS 
    | INTENT_DIRECT_MESSAGES 
    | INTENT_DIRECT_MESSAGE_REACTIONS 
    | INTENT_MESSAGE_CONTENT;