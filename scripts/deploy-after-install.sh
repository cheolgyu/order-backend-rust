#!/bin/bash

echo "=============deploy-after-install.sh===================="
#ls -al
#docker load -i /home/ec2-user/deploy-image/order.tar.gz
#docker run order
#/home/ec2-user/deploy/deploy.sh > /dev/null 2> /dev/null < /dev/null &
touch ~/hello-world.txt
sudo chmod +x /home/ec2-user/deploy/api/scripts/*.sh
cp /home/ec2-user/deploy/api/env-example /home/ec2-user/deploy/api/release/.env
cd /home/ec2-user/deploy/
/home/ec2-user/deploy/deploy.sh
#/home/ec2-user/deploy/deploy.sh > /dev/null 2> /dev/null < /dev/null &


echo "end====>deploy-after-install.sh"