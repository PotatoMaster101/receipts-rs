use aws_sdk_s3::operation::get_object::GetObjectError;
use aws_sdk_s3::operation::get_object::builders::GetObjectFluentBuilder;

#[derive(Debug, thiserror::Error)]
pub enum S3Error {
    #[error("stream error: {0}")]
    Stream(String),

    #[error("get error: {0}")]
    Get(#[from] Box<aws_sdk_s3::error::SdkError<GetObjectError>>),
}

impl From<aws_sdk_s3::error::SdkError<GetObjectError>> for S3Error {
    fn from(err: aws_sdk_s3::error::SdkError<GetObjectError>) -> Self {
        Self::Get(Box::new(err))
    }
}

#[derive(Clone, Debug)]
pub struct S3Info {
    pub bucket: String,
    pub key: String,
}

impl std::fmt::Display for S3Info {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "s3://{}/{}", &self.bucket, &self.key)
    }
}

impl S3Info {
    #[inline]
    pub fn new(bucket: String, key: String) -> Self {
        Self { bucket, key }
    }
}

#[derive(Clone, Debug)]
pub struct S3Service {
    pub client: aws_sdk_s3::Client,
}

impl S3Service {
    #[inline]
    pub async fn get_obj(&self, info: &S3Info) -> Result<Vec<u8>, S3Error> {
        let output = self.get_obj_builder(info).send().await?;
        Ok(output.body.collect().await.map_err(|e| S3Error::Stream(e.to_string()))?.to_vec())
    }

    #[inline]
    pub async fn get_obj_bytes(&self, info: &S3Info, count: usize) -> Result<Vec<u8>, S3Error> {
        let output = self.get_obj_builder(info).range(format!("bytes=0-{count}")).send().await?;
        Ok(output.body.collect().await.map_err(|e| S3Error::Stream(e.to_string()))?.to_vec())
    }

    #[inline]
    fn get_obj_builder(&self, info: &S3Info) -> GetObjectFluentBuilder {
        self.client.get_object().bucket(&info.bucket).key(&info.key)
    }
}

#[cfg(test)]
mod s3_info_tests {
    use super::*;

    #[test]
    fn test_new() {
        let sut = S3Info::new("test".to_string(), "test.jpg".to_string());
        assert_eq!(sut.bucket, "test");
        assert_eq!(sut.key, "test.jpg");

        let sut = S3Info::new("test2".to_string(), "test2.txt".to_string());
        assert_eq!(sut.bucket, "test2");
        assert_eq!(sut.key, "test2.txt");
    }

    #[test]
    fn test_display() {
        let sut = S3Info::new("test".to_string(), "test.jpg".to_string());
        assert_eq!(sut.to_string(), "s3://test/test.jpg");

        let sut = S3Info::new("test2".to_string(), "test2.txt".to_string());
        assert_eq!(sut.to_string(), "s3://test2/test2.txt");
    }
}

#[cfg(test)]
mod s3_service_tests {
    use super::*;
    use aws_sdk_s3::operation::get_object::GetObjectOutput;
    use aws_sdk_s3::primitives::ByteStream;
    use aws_smithy_mocks::{mock, mock_client};

    #[tokio::test]
    async fn test_get_obj() {
        let mock = mock!(aws_sdk_s3::Client::get_object)
            .then_output(move || GetObjectOutput::builder().body(ByteStream::from(b"test data".to_vec())).build());
        let sut = S3Service {
            client: mock_client!(aws_sdk_s3, [&mock]),
        };
        let result = sut.get_obj(&S3Info::new("bucket".to_string(), "file.txt".to_string())).await;
        assert!(result.is_ok());
        assert_eq!(mock.num_calls(), 1);
    }

    #[tokio::test]
    async fn test_get_obj_bytes() {
        let range = "bytes=0-31";
        let mock = mock!(aws_sdk_s3::Client::get_object)
            .match_requests(move |req| req.range().unwrap() == range)
            .then_output(move || GetObjectOutput::builder().body(ByteStream::from(b"test data".to_vec())).build());
        let sut = S3Service {
            client: mock_client!(aws_sdk_s3, [&mock]),
        };
        let result = sut.get_obj_bytes(&S3Info::new("bucket".to_string(), "file.txt".to_string()), 31).await;
        assert!(result.is_ok());
        assert_eq!(mock.num_calls(), 1);
    }
}
