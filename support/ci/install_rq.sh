#!/bin/bash
set -eu

echo "Installing rq"
curl -o /usr/local/bin/rq https://s3-eu-west-1.amazonaws.com/record-query/record-query/x86_64-unknown-linux-musl/rq
chmod +x /usr/local/bin/rq
