#!/bin/bash

echo "deploy-after-install.sh"
ls -al
docker load -i /home/ec2-user/deploy-image/order.tar.gz
docker run order
#/home/ec2-user/deploy/deploy.sh > /dev/null 2> /dev/null < /dev/null &
echo "end====>deploy-after-install.sh"