[![Rust](https://github.com/PotatoMaster101/receipts-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/PotatoMaster101/receipts-rs/actions/workflows/rust.yml)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

# Receipts-rs
Receipt app for playing with AWS.

## TODO
- [ ] Implement rest of backend
- [ ] Implement user authentication
- [ ] Implement file/text upload frontends

## Building
Install [`cargo lambda`](https://www.cargo-lambda.info/) and build the lambda functions:
```shell
cargo lambda build --arm64 --release --output-format zip
```

## Testing
```shell
cargo test
```

## Deploying
Install [`terraform`](https://developer.hashicorp.com/terraform) and provision AWS:
```shell
aws login
terraform -chdir=terraform init
terraform -chdir=terraform apply -auto-approve
```

## Destroy
```shell
terraform -chdir=terraform destroy -auto-approve
```
