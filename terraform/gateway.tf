resource "aws_apigatewayv2_api" "receipts" {
  name          = "${var.project}-${var.environment}"
  protocol_type = "HTTP"
}

resource "aws_apigatewayv2_integration" "process_receipt_text" {
  api_id           = aws_apigatewayv2_api.receipts.id
  integration_type = "AWS_PROXY"
  integration_uri  = aws_lambda_function.process_receipt_text.invoke_arn
}

resource "aws_apigatewayv2_route" "process_receipt_text" {
  api_id    = aws_apigatewayv2_api.receipts.id
  route_key = "POST /receipt"
  target    = "integrations/${aws_apigatewayv2_integration.process_receipt_text.id}"
}

resource "aws_apigatewayv2_stage" "process_receipt_text" {
  api_id      = aws_apigatewayv2_api.receipts.id
  auto_deploy = true
  name        = var.environment

  stage_variables = {
    "env" = var.environment
  }
}

resource "aws_lambda_permission" "process_receipt_text" {
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.process_receipt_text.function_name
  principal     = "apigateway.amazonaws.com"
  source_arn    = "${aws_apigatewayv2_api.receipts.execution_arn}/*/*"
  statement_id  = "AllowExecutionFromAPIGateway"
}
