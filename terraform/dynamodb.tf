resource "aws_dynamodb_table" "receipts" {
  billing_mode = "PAY_PER_REQUEST"
  hash_key     = "id"
  name         = "${var.project}-${var.environment}"
  range_key    = "process_time"

  attribute {
    name = "id"
    type = "S"
  }

  attribute {
    name = "process_time"
    type = "N"
  }

  point_in_time_recovery {
    enabled = true
  }

  server_side_encryption {
    enabled = true
  }

  ttl {
    attribute_name = "ttl"
    enabled        = true
  }
}
