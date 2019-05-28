#!/bin/bash

echo "deploy-after-install.sh"
touch ~/hello-world.txt
sudo chmod +x /home/ec2-user/deploy/api/scripts/*.sh
cp /home/ec2-user/deploy/api/env-example /home/ec2-user/deploy/api/release/.env
cd /home/ec2-user/deploy/
/home/ec2-user/deploy/deploy.sh
#/home/ec2-user/deploy/deploy.sh > /dev/null 2> /dev/null < /dev/null &
echo "end====>deploy-after-install.sh"