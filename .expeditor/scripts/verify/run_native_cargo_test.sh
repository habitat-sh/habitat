#!/bin/bash

set -eou pipefail

export DEBIAN_FRONTEND=noninteractive
apt-get update
apt-get install -y ca-certificates gcc libc6-dev wget openssl make pkg-config libzmq3-dev curl cmake
cargo test --no-fail-fast