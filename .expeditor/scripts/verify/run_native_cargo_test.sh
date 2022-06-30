#!/bin/bash

set -eou pipefail

sudo apt-get update
sudo DEBIAN_FRONTEND=noninteractive apt-get install -y ca-certificates gcc libc6-dev wget openssl make pkg-config libzmq3-dev curl
cargo test