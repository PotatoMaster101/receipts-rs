use aws_config::SdkConfig;
use utils::aws::dynamo::DynamoService;
use utils::aws::s3::S3Service;
use utils::aws::textract::TextractService;

#[derive(Clone, Debug)]
pub struct State {
    pub dynamo: DynamoService,
    pub s3: S3Service,
    pub textract: TextractService,
    pub receipt_table: String,
}

impl State {
    #[inline]
    pub fn new(config: SdkConfig, receipt_table: String) -> Self {
        Self {
            dynamo: DynamoService {
                client: aws_sdk_dynamodb::Client::new(&config),
            },
            s3: S3Service {
                client: aws_sdk_s3::Client::new(&config),
            },
            textract: TextractService {
                client: aws_sdk_textract::Client::new(&config),
            },
            receipt_table,
        }
    }
}
