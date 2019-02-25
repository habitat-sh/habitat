#!/bin/bash

set -eou pipefail

#
# Install core rust toolchain
# defaults to "stable"
#
install_rust_toolchain() {
  toolchain="${1?toolchain argument required}"

  echo "--- :rust: Installing rustup"
  curl https://sh.rustup.rs -sSf | sh -s -- --no-modify-path -y
  source "$HOME"/.cargo/env
  echo "--- :rust: Installing rust $toolchain"
  rustup toolchain install "$toolchain"  
}
