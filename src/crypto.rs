use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use worker::Headers;

use crate::error::{Error, BotResult};

pub (crate) fn has_signature(headers: &Headers) -> bool {
    headers.has("X-Signature-Ed25519").unwrap_or(false) && 
    headers.has("X-Signature-Timestamp").unwrap_or(false)
}

pub (crate) fn verify_signature(headers: &Headers, body: &[u8], public_key_hex: &str) -> BotResult<bool> {
    let signature_header = headers.get("X-Signature-Ed25519")?
        .ok_or_else(|| Error::MissingHeader("Missing X-Signature-Ed25519 header".into()))?;

    let timestamp_header = headers.get("X-Signature-Timestamp")?
        .ok_or_else(|| Error::MissingHeader("Missing X-Signature-Timestamp header".into()))?;

    let public_key_bytes = hex::decode(public_key_hex)
        .map_err(Error::ParseHexFailed)?;

    let verifying_key = VerifyingKey::from_bytes(&public_key_bytes.try_into().unwrap_or([0; 32]))
        .map_err(Error::CryptoError)?;

    let signature_bytes = hex::decode(signature_header)
        .map_err(Error::ParseHexFailed)?;

    let signature = Signature::from_bytes(&signature_bytes.try_into().unwrap_or([0; 64]));

    let mut message = Vec::with_capacity(timestamp_header.len() + body.len());
    message.extend_from_slice(timestamp_header.as_bytes());
    message.extend_from_slice(body);

    match verifying_key.verify(&message, &signature) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false), 
    }
}