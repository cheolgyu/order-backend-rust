#!/bin/bash
echo "=============docker-run.sh===================="

aws configure set default.region ap-northeast-2
$(aws ecr get-login --no-include-email)

sudo chmod +x /home/ec2-user/deploy/api/scripts/*.sh
echo "/home/ec2-user/deploy/api/scripts/blue-greend.sh"
ls -al
sudo /home/ec2-user/deploy/api/scripts/blue-greend.sh
