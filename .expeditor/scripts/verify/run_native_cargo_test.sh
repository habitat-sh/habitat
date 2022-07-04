#!/bin/bash
set -eou pipefail
# The test user as which the tests will be run
export TEST_USER=ubuntu
# Set filesystem root to a temporary folder that does not require sudo privileges
export FS_ROOT=/tmp/hab-test
# This is required to enable non-interactive installation of timezone data
export DEBIAN_FRONTEND=noninteractive

# Ensure all build and test dependencies are installed
apt-get update && apt-get install -y ca-certificates sudo gcc libc6-dev wget openssl make pkg-config libzmq3-dev curl cmake

# Create the test user and fix permissions
useradd -rm -d /home/$TEST_USER -s /bin/bash -g root -G sudo -u 1001 $TEST_USER
chown -R ubuntu:root /workdir

# Build binaries to be used in integration test
sudo -u $TEST_USER -H -E --preserve-env=PATH bash -c "cargo build"

# Run all test cases without stopping for failures
sudo -u $TEST_USER -H -E --preserve-env=PATH bash -c "cargo test --no-fail-fast"