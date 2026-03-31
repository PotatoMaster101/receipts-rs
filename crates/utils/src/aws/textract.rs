use crate::aws::s3::S3Info;
use crate::receipt::{LineItem, ReceiptDocument};
use aws_sdk_textract::operation::analyze_expense::AnalyzeExpenseError;
use aws_sdk_textract::types::{Document, ExpenseDocument, ExpenseField, S3Object};

#[derive(Debug, thiserror::Error)]
pub enum TextractError {
    #[error("expense error: {0}")]
    Expense(#[from] aws_sdk_textract::error::SdkError<AnalyzeExpenseError>),
}

#[derive(Clone, Debug)]
pub struct TextractService {
    pub client: aws_sdk_textract::Client,
}

impl TextractService {
    pub async fn process_receipt(&self, info: &S3Info) -> Result<Vec<ReceiptDocument>, TextractError> {
        let s3 = S3Object::builder().bucket(&info.bucket).name(&info.key).build();
        let doc = Document::builder().s3_object(s3).build();
        let response = self.client.analyze_expense().document(doc).send().await?;
        let receipts = response.expense_documents().iter().map(Self::get_receipt).collect();
        Ok(receipts)
    }

    fn get_receipt(doc: &ExpenseDocument) -> ReceiptDocument {
        let items = doc.line_item_groups().iter().flat_map(|g| g.line_items()).map(|l| {
            let f = |t| find_field(l.line_item_expense_fields(), t);
            LineItem {
                sku: f("PRODUCT_CODE"),
                name: f("ITEM"),
                price: f("PRICE").and_then(|s| s.parse::<f64>().ok()),
                unit_price: f("UNIT_PRICE").and_then(|s| s.parse::<f64>().ok()),
                quantity: f("QUANTITY").and_then(|s| s.parse::<i32>().ok()),
            }
        });

        let f = |t| find_field(doc.summary_fields(), t);
        ReceiptDocument {
            vendor: f("VENDOR_NAME"),
            date: f("INVOICE_RECEIPT_DATE"),
            items: items.collect(),
        }
    }
}

#[inline]
fn find_field(fields: impl AsRef<[ExpenseField]>, r#type: &str) -> Option<String> {
    fields
        .as_ref()
        .iter()
        .find(|f| f.r#type().and_then(|t| t.text()).is_some_and(|t| t == r#type))
        .and_then(|f| f.value_detection()?.text())
        .map(|s| s.to_string())
}
