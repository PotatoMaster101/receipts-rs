locals {
  policies = {
    lambda = aws_iam_policy.process_receipt_image.arn
    logs   = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
  }
  process_receipt_image_name = "${var.project}-${var.environment}-process-receipt-image"
  process_receipt_text_name  = "${var.project}-${var.environment}-process-receipt-text"
}

resource "aws_lambda_function" "process_receipt_image" {
  architectures    = ["arm64"]
  filename         = var.process_receipt_image_path
  function_name    = local.process_receipt_image_name
  handler          = "bootstrap"
  memory_size      = 128
  role             = aws_iam_role.process_receipt_image.arn
  runtime          = "provided.al2023"
  source_code_hash = filebase64sha256(var.process_receipt_image_path)
  timeout          = 30

  environment {
    variables = {
      DYNAMODB_RECEIPT_TABLE = aws_dynamodb_table.receipts.name
    }
  }
}

resource "aws_lambda_permission" "process_receipt_image" {
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.process_receipt_image.function_name
  principal     = "s3.amazonaws.com"
  source_arn    = aws_s3_bucket.receipt_images.arn
  statement_id  = "AllowS3Invoke"
}

resource "aws_iam_policy" "process_receipt_image" {
  name = local.process_receipt_image_name
  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action = [
          "dynamodb:GetItem",
          "dynamodb:PutItem",
          "dynamodb:Query",
          "dynamodb:UpdateItem",
        ],
        Effect   = "Allow"
        Resource = aws_dynamodb_table.receipts.arn
      },
      {
        Action = [
          "logs:CreateLogGroup",
          "logs:CreateLogStream",
          "logs:PutLogEvents",
        ],
        Effect   = "Allow",
        Resource = "arn:aws:logs:*:*:*"
      },
      {
        Action   = ["s3:GetObject"]
        Effect   = "Allow"
        Resource = ["${aws_s3_bucket.receipt_images.arn}/*"]
      },
      {
        Action   = ["textract:AnalyzeExpense"]
        Effect   = "Allow"
        Resource = "*"
      },
    ]
  })
}

resource "aws_iam_role" "process_receipt_image" {
  name = local.process_receipt_image_name

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action    = "sts:AssumeRole"
        Effect    = "Allow"
        Principal = { Service = "lambda.amazonaws.com" }
      }
    ]
  })
}

resource "aws_iam_role_policy_attachment" "process_receipt_image" {
  for_each   = local.policies
  policy_arn = each.value
  role       = aws_iam_role.process_receipt_image.name
}

resource "aws_iam_role_policy_attachments_exclusive" "process_receipt_image" {
  policy_arns = values(local.policies)
  role_name   = aws_iam_role.process_receipt_image.name
}

resource "aws_cloudwatch_log_group" "process_receipt_image" {
  name              = "/aws/lambda/${aws_lambda_function.process_receipt_image.function_name}"
  retention_in_days = 14
}

resource "aws_lambda_function" "process_receipt_text" {
  architectures    = ["arm64"]
  filename         = var.process_receipt_text_path
  function_name    = local.process_receipt_text_name
  handler          = "bootstrap"
  memory_size      = 128
  role             = aws_iam_role.process_receipt_text.arn
  runtime          = "provided.al2023"
  source_code_hash = filebase64sha256(var.process_receipt_text_path)
  timeout          = 30

  environment {
    variables = {
      DYNAMODB_RECEIPT_TABLE = aws_dynamodb_table.receipts.name
    }
  }
}

resource "aws_iam_policy" "process_receipt_text" {
  name = local.process_receipt_text_name
  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action = [
          "dynamodb:GetItem",
          "dynamodb:PutItem",
          "dynamodb:Query",
          "dynamodb:UpdateItem",
        ],
        Effect   = "Allow"
        Resource = aws_dynamodb_table.receipts.arn
      },
      {
        Action = [
          "logs:CreateLogGroup",
          "logs:CreateLogStream",
          "logs:PutLogEvents",
        ],
        Effect   = "Allow",
        Resource = "arn:aws:logs:*:*:*"
      },
    ]
  })
}

resource "aws_iam_role" "process_receipt_text" {
  name = local.process_receipt_text_name

  assume_role_policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Action    = "sts:AssumeRole"
        Effect    = "Allow"
        Principal = { Service = "lambda.amazonaws.com" }
      }
    ]
  })
}

resource "aws_iam_role_policy_attachment" "process_receipt_text" {
  for_each   = local.policies
  policy_arn = each.value
  role       = aws_iam_role.process_receipt_text.name
}

resource "aws_iam_role_policy_attachments_exclusive" "process_receipt_text" {
  policy_arns = values(local.policies)
  role_name   = aws_iam_role.process_receipt_text.name
}

resource "aws_cloudwatch_log_group" "process_receipt_text" {
  name              = "/aws/lambda/${aws_lambda_function.process_receipt_text.function_name}"
  retention_in_days = 14
}
