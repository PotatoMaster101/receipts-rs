mod aws;

use crate::aws::State;
use aws_config::{BehaviorVersion, load_defaults};
use lambda_http::tracing::info;
use lambda_http::{Body, Request, Response, run};
use lambda_runtime::{Error, service_fn, tracing};
use std::sync::Arc;

async fn handler(event: Request, _state: Arc<State>) -> Result<Response<Body>, Error> {
    let text = match event.body() {
        Body::Text(text) => text.to_string(),
        Body::Binary(blob) => String::from_utf8(blob.to_vec()).unwrap_or_default(),
        _ => "".to_string(),
    };
    info!("Received data: {}", text);

    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body("{\"status\": \"processed\"}".into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();
    let db_name = std::env::var("DYNAMODB_RECEIPT_TABLE").expect("DYNAMODB_RECEIPT_TABLE must be set");
    let config = load_defaults(BehaviorVersion::latest()).await;
    let state = Arc::new(State::new(config, db_name));
    let lambda = |event| handler(event, state.clone());
    run(service_fn(lambda)).await
}
