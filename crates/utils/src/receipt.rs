use crate::aws::s3::S3Info;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ReceiptSource {
    S3 { bucket: String, key: String },
    Text { text: String },
}

impl ReceiptSource {
    pub fn generate_id(&self) -> String {
        match self {
            ReceiptSource::S3 { bucket, key } => format!("s3://{}/{}", bucket, key),
            ReceiptSource::Text { text } => format!("text://{}", hex::encode(Sha256::digest(text.as_bytes()))),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LineItem {
    pub sku: Option<String>,
    pub name: Option<String>,
    pub price: Option<f64>,
    pub unit_price: Option<f64>,
    pub quantity: Option<i32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReceiptDocument {
    pub vendor: Option<String>,
    pub date: Option<String>,
    pub items: Vec<LineItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProcessedReceipt {
    pub id: String,
    pub source: ReceiptSource,
    pub receipts: Vec<ReceiptDocument>,
    pub process_time: i64,
    pub ttl: i64,
}

impl ProcessedReceipt {
    pub fn from_s3(s3: S3Info, receipts: impl IntoIterator<Item = ReceiptDocument>) -> Self {
        let source = ReceiptSource::S3 {
            bucket: s3.bucket,
            key: s3.key,
        };
        Self::new(source, receipts)
    }

    pub fn from_text(text: impl Into<String>, receipts: impl IntoIterator<Item = ReceiptDocument>) -> Self {
        let mut text = text.into();
        if let Some((idx, _)) = text.char_indices().nth(10000) {
            text.truncate(idx);
        }
        Self::new(ReceiptSource::Text { text }, receipts)
    }

    fn new(source: ReceiptSource, receipts: impl IntoIterator<Item = ReceiptDocument>) -> Self {
        let now = chrono::Utc::now().timestamp();
        Self {
            id: source.generate_id(),
            source,
            receipts: receipts.into_iter().collect(),
            process_time: now,
            ttl: now + (3650 * 24 * 60 * 60),
        }
    }
}

#[cfg(test)]
mod receipt_source_tests {
    use super::*;

    #[test]
    fn test_generate_id() {
        let sut = ReceiptSource::S3 {
            bucket: "test".to_string(),
            key: "test.jpg".to_string(),
        };
        assert_eq!(sut.generate_id(), "s3://test/test.jpg");

        let sut = ReceiptSource::Text { text: "test".to_string() };
        assert_eq!(
            sut.generate_id(),
            "text://9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08"
        );
    }
}

#[cfg(test)]
mod processed_receipt_tests {
    use super::*;

    #[test]
    fn test_new() {
        let sut = ProcessedReceipt::new(
            ReceiptSource::S3 {
                bucket: "test".to_string(),
                key: "test.jpg".to_string(),
            },
            [],
        );
        assert_eq!(
            sut.source,
            ReceiptSource::S3 {
                bucket: "test".to_string(),
                key: "test.jpg".to_string(),
            }
        );
        assert_eq!(sut.receipts.len(), 0);

        let sut = ProcessedReceipt::new(
            ReceiptSource::Text { text: "test".to_string() },
            [ReceiptDocument {
                vendor: Some("abc".to_string()),
                date: Some("def".to_string()),
                items: vec![],
            }],
        );
        assert_eq!(sut.source, ReceiptSource::Text { text: "test".to_string() });
        assert_eq!(sut.receipts.len(), 1);
    }

    #[test]
    fn test_from_s3() {
        let info = S3Info::new("test".to_string(), "test.jpg".to_string());
        let sut = ProcessedReceipt::from_s3(info.clone(), []);
        assert_eq!(
            sut.source,
            ReceiptSource::S3 {
                bucket: info.bucket.clone(),
                key: info.key.clone(),
            }
        );
        assert_eq!(sut.receipts.len(), 0);
        assert!(sut.process_time > 0);
        assert!(sut.ttl > sut.process_time);

        let sut = ProcessedReceipt::from_s3(
            info.clone(),
            [ReceiptDocument {
                vendor: Some("abc".to_string()),
                date: Some("def".to_string()),
                items: vec![],
            }],
        );
        assert_eq!(sut.receipts.len(), 1);
        assert!(sut.process_time > 0);
        assert!(sut.ttl > sut.process_time);
    }

    #[test]
    fn test_from_text() {
        let sut = ProcessedReceipt::from_text("test", []);
        assert_eq!(sut.source, ReceiptSource::Text { text: "test".to_string() });
        assert_eq!(sut.receipts.len(), 0);

        let sut = ProcessedReceipt::from_text(
            "test",
            [ReceiptDocument {
                vendor: Some("abc".to_string()),
                date: Some("def".to_string()),
                items: vec![],
            }],
        );
        assert_eq!(sut.receipts.len(), 1);
        assert!(sut.process_time > 0);
        assert!(sut.ttl > sut.process_time);

        let sut = ProcessedReceipt::from_text("a".repeat(20000), []);
        assert_eq!(sut.source, ReceiptSource::Text { text: "a".repeat(10000) });
    }
}
