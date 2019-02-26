#!/bin/bash

set -euo pipefail

export RUSTUP_HOME="/opt/rust"
export CARGO_HOME="/home/buildkite-agent/.cargo"
export PATH="/opt/rust/bin:$PATH"

rustup="/opt/rust/bin/rustup"

# NOTE: this line should be deleted after the Docker container gets updated
sudo chown -R buildkite-agent /home/buildkite-agent

source ./install-nightly-rustfmt.sh
