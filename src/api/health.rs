use serde_json::json;
use worker::{Env, Response};

pub async fn health(_env: Env, _token: String) -> worker::Result<Response> {
    Response::from_json(&json!({
        "status" : "healthy"
    }))
}