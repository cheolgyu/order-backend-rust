#!/bin/bash
echo "=============docker-run.sh===================="

aws configure set default.region ap-northeast-2
$(aws ecr get-login --no-include-email)
ls -al
sudo chmod +x ./*.sh
