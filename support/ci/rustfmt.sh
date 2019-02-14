#!/bin/bash

set -euo pipefail

export RUSTUP_HOME="/opt/rust"
export CARGO_HOME="/home/buildkite-agent/.cargo"
export PATH="/opt/rust/bin:$PATH"

rustup="/opt/rust/bin/rustup"

# This is being pinned to a specific date because sometimes nightly will make a change that breaks rustfmt's usage of rustc.
# We should decide how/when/if we want to advance this date pin.
toolchain="nightly-2019-02-11"

# NOTE: this line should be deleted after the Docker container gets updated
sudo chown -R buildkite-agent /home/buildkite-agent

sudo -E $rustup toolchain install $toolchain
sudo -E $rustup component add --toolchain $toolchain rustfmt

cargo_fmt="$rustup run $toolchain cargo fmt --all -- --check"
echo "--- Running cargo fmt command: $cargo_fmt"
$cargo_fmt
