use serde_json::json;
use worker::{Env, Response, WorkerVersionMetadata};

pub async fn version(env: Env) -> worker::Result<Response> {
    let Some(metadata_binding) = env.var("WORKER_METADATA_BINDING").ok() else {
        worker::console_warn!("WORKER_METADATA_BINDING env variable not set!");
        return Response::builder().with_status(500).from_json(&json!({
            "status": "error",
            "message" : "Could not get version metadata"
        }))
    };

    let metadata: WorkerVersionMetadata = env.get_binding::<WorkerVersionMetadata>(
        &metadata_binding.to_string()
    )?;

    let timestamp = metadata.timestamp();
    let id = metadata.id();
    let tag = metadata.tag();
    
    Response::from_json(&json!({
        "timestamp" : timestamp,
        "id" : id,
        "tag" : tag
    }))
}