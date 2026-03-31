use crate::receipt::ProcessedReceipt;
use aws_sdk_dynamodb::error::SdkError;
use aws_sdk_dynamodb::operation::{get_item, put_item};

#[derive(Debug, thiserror::Error)]
pub enum DynamoError {
    #[error("serialization error: {0}")]
    Serialize(#[from] serde_dynamo::Error),

    #[error("put error: {0}")]
    Put(#[from] SdkError<put_item::PutItemError>),

    #[error("get error: {0}")]
    Get(#[from] SdkError<get_item::GetItemError>),
}

#[derive(Clone, Debug)]
pub struct DynamoService {
    pub client: aws_sdk_dynamodb::Client,
}

impl DynamoService {
    pub async fn put_receipt(&self, receipt: &ProcessedReceipt, table: &str) -> Result<(), DynamoError> {
        let item = serde_dynamo::to_item(&receipt).map_err(DynamoError::Serialize)?;
        self.client.put_item().table_name(table).set_item(Some(item)).send().await?;
        Ok(())
    }

    pub async fn get_receipt(&self, id: &str, table: &str) -> Result<Option<ProcessedReceipt>, DynamoError> {
        let key_val = serde_dynamo::to_attribute_value(id).map_err(DynamoError::Serialize)?;
        let response = self.client.get_item().table_name(table).key("id", key_val).send().await?;
        match response.item {
            Some(item) => Ok(Some(serde_dynamo::from_item(item).map_err(DynamoError::Serialize)?)),
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aws_sdk_dynamodb::operation::get_item::GetItemOutput;
    use aws_sdk_dynamodb::operation::put_item::PutItemOutput;
    use aws_smithy_mocks::{mock, mock_client};

    #[tokio::test]
    async fn test_put_receipt() {
        let mock = mock!(aws_sdk_dynamodb::Client::put_item)
            .match_requests(|r| r.table_name() == Some("test-table"))
            .then_output(|| PutItemOutput::builder().build());
        let sut = DynamoService {
            client: mock_client!(aws_sdk_dynamodb, [&mock]),
        };
        let result = sut.put_receipt(&ProcessedReceipt::from_text("receipt", []), "test-table").await;
        assert!(result.is_ok());
        assert_eq!(mock.num_calls(), 1);
    }

    #[tokio::test]
    async fn test_get_receipt() {
        let expected = ProcessedReceipt::from_text("test", []);
        let mock = mock!(aws_sdk_dynamodb::Client::get_item).then_output(move || {
            GetItemOutput::builder()
                .set_item(Some(serde_dynamo::to_item(&expected).expect("to_item")))
                .build()
        });
        let sut = DynamoService {
            client: mock_client!(aws_sdk_dynamodb, [&mock]),
        };
        let result = sut.get_receipt("123", "test-table").await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
        assert_eq!(mock.num_calls(), 1);
    }
}
