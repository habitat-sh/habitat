#!/bin/bash

set -euo pipefail

maybe_install_rustup() {
  if command -v rustup && command -v cargo &>/dev/null; then
    echo "--- :rust: rustup is currently installed."
  else
    echo "--- :rust: Installing rustup."
    curl https://sh.rustup.rs -sSf | sh -s -- --no-modify-path -y
    source "$HOME"/.cargo/env
  fi
}

maybe_install_rust_toolchain() {
  local toolchain="${1?toolchain argument required}"

  if rustup component list --toolchain "$toolchain" >/dev/null 2>&1; then
    echo "--- :rust: Rust $toolchain is already installed."
  else
    echo "--- :rust: Installing rust $toolchain."
    rustup toolchain install "$toolchain"
  fi
}

# Due to the nature of nightly rust, sometimes changes will break rustfmt's
# usage of rustc. If this happens, nightly rust won't include rustfmt,
# and we need to automatically fall back to a version that does include it.
maybe_install_rustfmt() {
  local max_days=90

  for days_ago in $(seq 0 1 $max_days)
  do
    local date
    date=$(date -d "$days_ago days ago" +%Y-%m-%d)
    toolchain="nightly-$date"

    if maybe_install_rust_toolchain "$toolchain"; then
      echo "--- :rust: Installation of $toolchain succeeded, or it was already installed."
    else
      next_days=$((days_ago + 1))
      echo "--- :rust: Rust $toolchain doesn't exist. Let's try $next_days days(s) ago."
      continue
    fi

    if rustup component add --toolchain "$toolchain" rustfmt; then
      echo "--- :rust: Installation of rustfmt for $toolchain succeeded, or it was already up to date."
      return
    else
      next_days=$((days_ago + 1))
      echo "--- :rust: Rust $toolchain did not include rustfmt. Let's try $next_days day(s) ago."
    fi
  done

  echo "We couldn't find a release of nightly rust in the past $max_days days that includes rustfmt. Giving up entirely."
  exit 1
}
