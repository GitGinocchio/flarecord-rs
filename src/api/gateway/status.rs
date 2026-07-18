use worker::*;




pub async fn status(env: Env, _token: String) -> worker::Result<Response> {
    let gateway_do_binding = match env.var("WORKER_GATEWAY_DO_BINDING") {
        Err(e) => {
            worker::console_error!("{e}");
            return Err(e);
        }
        Ok(b) => b.to_string()
    };

    let do_namespace = env.durable_object(&gateway_do_binding)?;

    // TODO: Sostituire "test" con qualcos'altro...
    let gateway_stub = do_namespace.get_by_name("test")?;

    let mut init = RequestInit::new();
    init.with_method(Method::Get);

    let request = Request::new_with_init("https://0.0.0.0/status", &init)?;

    gateway_stub.fetch_with_request(request).await
}