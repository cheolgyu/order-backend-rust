#!/bin/bash -e

docker images


REGISTRY_URL=410450153592.dkr.ecr.ap-northeast-2.amazonaws.com
SOURCE_IMAGE="order-backend-rust"
TARGET_IMAGE="${REGISTRY_URL}/${SOURCE_IMAGE}"
TARGET_IMAGE_LATEST="${TARGET_IMAGE}:latest"
TIMESTAMP=$(date '+%Y%m%d%H%M%S')
VERSION="${TIMESTAMP}-${TRAVIS_COMMIT}"
TARGET_IMAGE_VERSIONED="${TARGET_IMAGE}:${VERSION}"

#aws configure set default.region ${EB_REGION}

#$(aws ecr get-login --no-include-email)

# update latest version
docker tag ${SOURCE_IMAGE} ${TARGET_IMAGE_LATEST}
docker push ${TARGET_IMAGE_LATEST}

# push new version
docker tag ${SOURCE_IMAGE} ${TARGET_IMAGE_VERSIONED}
docker push ${TARGET_IMAGE_VERSIONED}



