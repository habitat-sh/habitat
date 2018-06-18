#!/bin/sh
set -eux

# Install Rust
curl -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
. $HOME/.cargo/env
# cargo install protobuf
rustc --version
cargo --version

npm install -g docco
echo "Node $(node --version)"
echo "npm $(npm --version)"
echo "docco $(docco --version)"

sh /tmp/install.sh
rm -rf /tmp/install.sh
