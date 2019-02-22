#!/bin/bash

set -euo pipefail

export RUSTUP_HOME="/opt/rust"
export CARGO_HOME="/home/buildkite-agent/.cargo"
export PATH="/opt/rust/bin:$PATH"

rustup="/opt/rust/bin/rustup"

# NOTE: this line should be deleted after the Docker container gets updated
sudo chown -R buildkite-agent /home/buildkite-agent

# Due to the nature of nightly rust, sometimes changes will break rustfmt's
# usage of rustc. If this happens, nightly rust won't include rustfmt,
# and we need to automatically fall back to a version that does include it.
# Note that we begin with 1 day ago, since nightly packages can sometimes not
# exist when this script runs if we leave it at 0.
max_days=90

for days_ago in $(seq 1 1 $max_days)
do
  date=$(date -d "$days_ago days ago" +%Y-%m-%d)
  toolchain="nightly-$date"
  echo "Installing rust $toolchain"
  sudo -E $rustup toolchain install "$toolchain"

  if sudo -E $rustup component add --toolchain "$toolchain" rustfmt; then
    cargo_fmt="$rustup run $toolchain cargo fmt --all -- --check"
    echo "Running cargo fmt command: $cargo_fmt"
    $cargo_fmt
    exit
  else
    next_days=$((days_ago + 1))
    echo "Rust $toolchain did not include rustfmt. Let's try $next_days day(s) ago."
  fi
done

echo "We couldn't find a release of nightly rust in the past $max_days days that includes rustfmt. Giving up entirely."
exit 1
