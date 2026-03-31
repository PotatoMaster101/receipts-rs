mod aws;

use aws_config::{BehaviorVersion, load_defaults};
use lambda_runtime::{Error, run, service_fn, tracing};

async fn handler() -> Result<(), Error> {
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();
    let db_name = std::env::var("DYNAMODB_RECEIPT_TABLE").expect("DYNAMODB_RECEIPT_TABLE must be set");
    let config = load_defaults(BehaviorVersion::latest()).await;
    //run(service_fn(handler)).await
    Ok(())
}
