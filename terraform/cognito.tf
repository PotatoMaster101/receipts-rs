resource "aws_cognito_user_pool" "receipt_users" {
  name = "${var.project}-${var.environment}"

  auto_verified_attributes = ["email"]
  username_attributes      = ["email"]

  password_policy {
    minimum_length    = 12
    require_lowercase = true
    require_numbers   = true
  }
}

resource "aws_cognito_user_pool_domain" "receipt_users" {
  domain       = "${var.project}-${var.environment}"
  user_pool_id = aws_cognito_user_pool.receipt_users.id
}
