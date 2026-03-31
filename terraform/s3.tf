locals {
  image_extensions = [".jpg", ".jpeg", ".png"]
}

resource "aws_s3_bucket" "receipt_images" {
  bucket_prefix = "${var.project}-${var.environment}-"
  force_destroy = true
}

resource "aws_s3_bucket_public_access_block" "receipt_images" {
  bucket = aws_s3_bucket.receipt_images.id

  block_public_acls       = true
  block_public_policy     = true
  ignore_public_acls      = true
  restrict_public_buckets = true
}

resource "aws_s3_bucket_notification" "receipt_images" {
  depends_on = [aws_lambda_permission.process_receipt_image]
  bucket     = aws_s3_bucket.receipt_images.id

  dynamic "lambda_function" {
    for_each = local.image_extensions
    content {
      lambda_function_arn = aws_lambda_function.process_receipt_image.arn
      events              = ["s3:ObjectCreated:*"]
      filter_suffix       = lambda_function.value
    }
  }
}
