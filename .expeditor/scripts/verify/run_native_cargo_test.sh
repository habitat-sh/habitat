#!/bin/bash

set -eou pipefail

sudo apt-get update
sudo apt-get install -y ca-certificates gcc libc6-dev wget openssl make pkg-config libzmq3-dev curl
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh -s -- -y
cargo test