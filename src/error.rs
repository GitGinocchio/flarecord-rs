use hex::FromHexError;
use serde_json::json;
use thiserror::Error;
use worker::Response;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Lock poisoned")]
    LockPoisoned,

    #[error("Header '{0}' not found.")]
    MissingHeader(String),

    #[error("Option '{0}' not found.")]
    MissingOption(String),

    #[error("Invalid payload: {0}")]
    InvalidPayload(String),

    #[error("Invalid interaction: {0}")]
    InvalidInteraction(String),

    #[error("Invalid option name: {0}")]
    InvalidOptionName(String),

    #[error("Invalid option type: {0}")]
    InvalidOptionType(String),

    #[error("Error resolving value: {0}")]
    ResolveError(String),

    #[error("Invalid public key or signature format: {0:?}")]
    CryptoError(#[from] ed25519_dalek::SignatureError),

    #[error("Failed to parse from hex: {0:?}")]
    ParseHexFailed(#[from] FromHexError),

    #[error("JSON error: {0:?}")]
    JsonFailed(#[from] serde_json::Error),

    #[error("Worker error: {0}")]
    WorkerError(#[from] worker::Error),

    #[error("Reqwest error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error("Environment variable '{0}' not found.")]
    EnvironmentVariableNotFound(String),

    #[error("Command '{0}' is not registered")]
    CommandNotFound(String),

    #[error("Option '{0}' not found")]
    OptionNotFound(String),

    #[error("Modal '{0}' is not registered")]
    ModalNotFound(String),

    #[error("Component '{0}' is not registered")]
    ComponentNotFound(String),

    #[error("Execute not implemented for command: '{0}'")]
    ExecuteNotImplemented(String),

    #[error("Autocomplete not implemented for comand: '{0}'")]
    AutocompleteNotImplemented(String),

    #[error("Error communicating with {0}")]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),

    #[error("Error: {0}")]
    Generic(String),
}

impl Error {
    pub fn as_response(self) -> worker::Result<Response> {
        worker::console_error!("Error: {self:?}");

        match self {
            Self::ReqwestError(error) => {
                let status = error.status().map(|s| s.as_u16());

                Response::builder()
                    .with_status(status.unwrap_or(500))
                    .from_json(&json!({
                        "status" : "error",
                        "code" : error.status().map(|s| s.as_str().to_string()),
                        "message" : error.to_string()
                    }))
            },
            _ => {
                Response::error(format!("Error: {:?}", self), 500)
            }
        }
    }
}

impl From<Error> for worker::Error {
    fn from(value: Error) -> Self {
        Self::from(format!("[rsflarecord] Error: {:?}", value))
    }
}

pub type BotResult<T> = std::result::Result<T, Error>;