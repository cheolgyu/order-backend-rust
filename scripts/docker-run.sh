#!/bin/bash
echo "=============docker-run.sh===================="

aws configure set default.region ap-northeast-2
$(aws ecr get-login --no-include-email)
# order-backend-rust
docker run 410450153592.dkr.ecr.ap-northeast-2.amazonaws.com/order-backend-rust:latest