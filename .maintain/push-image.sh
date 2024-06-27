#!/bin/bash

source_tag=$1
target_tag=$2

PROVIDER_BIN_NAME="dip-provider-node-template"
CONSUMER_BIN_NAME="dip-consumer-node-template"

# Function to tag and push images
tag_and_push() {
  local source=$1
  local target=$2
  local registry=$3

  docker tag $source $target
  docker push $target &
}

# Tag and push to Docker Hub
tag_and_push local/standalone-node:$source_tag ${DOCKER_HUB_STANDALONE}:$target_tag
tag_and_push local/kilt-node:$source_tag ${DOCKER_HUB_PARACHAIN}:$target_tag
tag_and_push local/$PROVIDER_BIN_NAME:$source_tag ${DOCKER_HUB_DIP_PROVIDER_TEMPLATE}:$target_tag
tag_and_push local/$CONSUMER_BIN_NAME:$source_tag ${DOCKER_HUB_DIP_CONSUMER_TEMPLATE}:$target_tag

# Tag and push to AWS
tag_and_push local/standalone-node:$source_tag $AWS_REGISTRY/kilt/prototype-chain:$target_tag
tag_and_push local/kilt-node:$source_tag $AWS_REGISTRY/kilt-parachain/collator:$target_tag
tag_and_push local/$PROVIDER_BIN_NAME:$source_tag $AWS_REGISTRY/$PROVIDER_BIN_NAME:$target_tag
tag_and_push local/$CONSUMER_BIN_NAME:$source_tag $AWS_REGISTRY/$CONSUMER_BIN_NAME:$target_tag

wait
