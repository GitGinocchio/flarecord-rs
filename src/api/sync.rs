use serde_json::json;
use twilight_model::id::Id;
use worker::{Env, Response};

use crate::{error::Error, services::discord::DiscordService};

pub async fn sync(env: Env, token: String) -> worker::Result<Response> {
    let application_id = env.secret("DISCORD_BOT_APPLICATION_ID")
        .map_err(|e| Error::EnvironmentVariableNotFound(format!("{e}")))?
        .to_string()
        .parse::<u64>()
        .map_err(|e| Error::Generic(format!("Parse int error: {e}")))?;

    let service = DiscordService::new(token);
    
    match service.update_global_commands(Id::new(application_id)).await {
        Err(e) => return e.as_response(),
        Ok(_) => {}
    };

    Response::from_json(&json!({
        "status" : "success"
    }))
}