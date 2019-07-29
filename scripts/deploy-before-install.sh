#!/bin/bash

echo "=============deploy-before-install.sh===================="
ls -al
now=$(date +"%m_%d_%Y")
echo now
mkdir now
sudo chown -R ec2-user:ec2-user /home/ec2-user/deploy

