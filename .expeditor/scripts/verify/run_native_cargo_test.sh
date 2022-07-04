#!/bin/bash
set -eou pipefail
export TEST_USER=ubuntu

export DEBIAN_FRONTEND=noninteractive
apt-get update
apt-get install -y ca-certificates sudo gcc libc6-dev wget openssl make pkg-config libzmq3-dev curl cmake

useradd -rm -d /home/$TEST_USER -s /bin/bash -g root -G sudo -u 1001 $TEST_USER

# Build binaries to be used in integration test
sudo -u $TEST_USER -H -E --preserve-env=PATH bash -c "cargo build"
sudo -u $TEST_USER -H -E --preserve-env=PATH bash -c "cargo test --no-fail-fast"