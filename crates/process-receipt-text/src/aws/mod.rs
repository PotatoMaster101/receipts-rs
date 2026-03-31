use aws_config::SdkConfig;
use utils::aws::dynamo::DynamoService;

#[derive(Clone, Debug)]
pub struct State {
    pub dynamo: DynamoService,
    pub receipt_table: String,
}

impl State {
    #[inline]
    pub fn new(config: SdkConfig, receipt_table: String) -> Self {
        Self {
            dynamo: DynamoService {
                client: aws_sdk_dynamodb::Client::new(&config),
            },
            receipt_table,
        }
    }
}
