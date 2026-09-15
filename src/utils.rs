use std::{any::TypeId, hash::{DefaultHasher, Hash, Hasher}};

use worker::Headers;

use crate::crypto::has_signature;

#[allow(unused)]
pub (crate) fn get_type_id<T: 'static>() -> String {
    let mut s = DefaultHasher::new();
    TypeId::of::<T>().hash(&mut s);
    let hash = s.finish();
    format!("{:016x}", hash)[..4].to_string()
}

pub (crate) fn get_id_from_type_id(type_id: TypeId) -> String {
    let mut s = DefaultHasher::new();
    type_id.hash(&mut s);
    let hash = s.finish();
    format!("{:016x}", hash)[..4].to_string()
}

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