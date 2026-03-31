use aws_sdk_bedrockruntime::operation::converse::ConverseError;
use aws_sdk_bedrockruntime::types::{ContentBlock, ConversationRole, Message};

#[derive(Debug, thiserror::Error)]
pub enum BedrockError {
    #[error("build error: {0}")]
    Build(#[from] aws_sdk_bedrockruntime::error::BuildError),

    #[error("converse error: {0}")]
    Converse(#[from] Box<aws_sdk_bedrockruntime::error::SdkError<ConverseError>>),

    #[error("empty response")]
    EmptyResponse,
}

impl From<aws_sdk_bedrockruntime::error::SdkError<ConverseError>> for BedrockError {
    fn from(err: aws_sdk_bedrockruntime::error::SdkError<ConverseError>) -> Self {
        Self::Converse(Box::new(err))
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BedrockModel {
    Nova2Lite,
}

impl std::fmt::Display for BedrockModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BedrockModel::Nova2Lite => write!(f, "amazon.nova-lite-v2:0"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct BedrockService {
    pub client: aws_sdk_bedrockruntime::Client,
}

impl BedrockService {
    pub async fn inference(&self, model: BedrockModel, input: String) -> Result<String, BedrockError> {
        let message = Message::builder()
            .role(ConversationRole::User)
            .content(ContentBlock::Text(input))
            .build()?;
        let response = self.client.converse().model_id(model.to_string()).messages(message).send().await?;
        let text = response
            .output()
            .and_then(|o| o.as_message().ok())
            .map(|m| {
                m.content()
                    .iter()
                    .filter_map(|c| c.as_text().ok())
                    .cloned()
                    .collect::<Vec<String>>()
                    .join("\n")
            })
            .ok_or(BedrockError::EmptyResponse)?;
        Ok(text)
    }
}
