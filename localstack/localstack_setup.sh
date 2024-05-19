#!/bin/bash

set -euo pipefail

queue_url="http://sqs.eu-west-1.localhost.localstack.cloud:4566/000000000000/eurorust"

echo "Deleting localstack queue if it already exists"
aws --endpoint-url=http://localhost:4566 sqs delete-queue --queue-url ${queue_url} > /dev/null

echo "Creating localstack sqs queue"
aws --endpoint-url=http://localhost:4566 sqs create-queue --queue-name eurorust > /dev/null

queues=$(aws --endpoint-url=http://localhost:4566 sqs list-queues --query 'QueueUrls[*]' --output text --)
echo "Queue urls: $queues"
