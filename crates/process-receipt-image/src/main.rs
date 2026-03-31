mod state;

use crate::state::State;
use anyhow::{Context, bail};
use aws_config::{BehaviorVersion, load_defaults};
use aws_lambda_events::s3::S3Event;
use lambda_runtime::tracing::error;
use lambda_runtime::{Error, LambdaEvent, run, service_fn, tracing};
use std::sync::Arc;
use utils::aws::s3::S3Info;
use utils::file;
use utils::receipt::ProcessedReceipt;

const MAX_FILE_SIZE: i64 = 4 * 1024 * 1024;

async fn handler(event: LambdaEvent<S3Event>, state: Arc<State>) -> Result<(), Error> {
    for record in event.payload.records {
        let result = async {
            let bucket = record.s3.bucket.name.as_deref().context("Missing bucket name")?;
            let key = record.s3.object.key.as_deref().context("Missing key")?;
            let size = record.s3.object.size.context("Missing size")?;
            let info = S3Info::new(bucket.to_string(), key.to_string());
            if size > MAX_FILE_SIZE {
                bail!("{info}: {size} bytes > {MAX_FILE_SIZE} bytes");
            }
            if !file::check_ext(key, &["jpg", "jpeg", "png"]) {
                bail!("{info}: not an image file");
            }

            let header = state.s3.get_obj_bytes(&info, 32).await?;
            if !file::check_kind(&header, &[file::FileKind::Jpeg, file::FileKind::Png]) {
                bail!("{info}: not an image file");
            }

            let receipts = state.textract.process_receipt(&info).await?;
            let processed = ProcessedReceipt::from_s3(info, receipts);
            state.dynamo.put_receipt(&processed, &state.receipt_table).await?;
            Ok::<(), anyhow::Error>(())
        };

        if let Err(err) = result.await {
            error!("{:#}", err);
        }
    }
    Ok(())
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
