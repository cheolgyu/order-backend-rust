#!/bin/bash
echo "/home/ec2-user/deploy/api/scripts/blue-greend.sh"
DOCKER_APP_NAME=api
EXIST_BLUE=$(docker-compose -p ${DOCKER_APP_NAME}-blue -f docker-compose.blue.yml ps | grep Up)

if [ -z "$EXIST_BLUE" ]; then
    echo "blue======> up start"
    docker-compose -p api-blue -f docker-compose.blue.yml up --build -d
    sleep 60
    echo "blue----->up end"
    docker-compose -p api-green -f docker-compose.green.yml down
   echo "green down"
else
    echo "green===========> up start"
    docker-compose -p api-green -f docker-compose.green.yml up --build -d
    sleep 60
    echo "green------> up end"
    docker-compose -p api-blue -f docker-compose.blue.yml down

    echo "blue down"
fi