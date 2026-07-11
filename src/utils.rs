use worker::Headers;

use crate::crypto::has_signature;



pub (crate) fn is_interaction(headers: &Headers) -> bool {
    if !has_signature(headers) {
        return false
    }

    match headers.get("user-agent") {
        Ok(Some(user_agent)) if user_agent == "Discord-Interactions/1.0 (+https://discord.com)" => true,
        Ok(_) | Err(_) => false
    }
}

pub (crate) fn has_api_access(headers: &Headers, expected_token: &str) -> bool {
    match headers.get("bot-token") {
        Ok(Some(bot_token)) if bot_token == expected_token => true,
        Ok(_) | Err(_) => false
    }
}