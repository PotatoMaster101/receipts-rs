variable "aws_region" { default = "ap-southeast-2" }
variable "environment" { default = "dev" }
variable "project" { default = "receipts" }

terraform {
  required_providers {
    aws = { source = "hashicorp/aws", version = "~> 6.41" }
  }
}

provider "aws" {
  region = var.aws_region
  default_tags {
    tags = {
      Environment = var.environment
      ManagedBy   = "terraform"
      Project     = var.project
    }
  }
}

module "main" {
  source                     = "./terraform"
  aws_region                 = var.aws_region
  environment                = var.environment
  process_receipt_image_path = "${path.root}/target/lambda/process-receipt-image/bootstrap.zip"
  process_receipt_text_path  = "${path.root}/target/lambda/process-receipt-text/bootstrap.zip"
  project                    = var.project
}
