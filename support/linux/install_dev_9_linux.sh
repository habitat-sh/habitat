#!/bin/sh
set -eux

# Install Rust and musl libc target
curl -sSf https://sh.rustup.rs \
  | env -u CARGO_HOME sh -s -- -y --no-modify-path --default-toolchain stable
env -u CARGO_HOME rustup target add x86_64-unknown-linux-musl
env -u CARGO_HOME cargo install protobuf
rustc --version
cargo --version

# Install Docker
curl -sSL https://get.docker.io | sh
docker --version

if [ ! -f /usr/bin/node ] && [ -f /usr/bin/nodejs ]; then
  ln -snf /usr/bin/nodejs /usr/bin/node
fi
npm install -g docco
echo "Node $(node --version)"
echo "npm $(npm --version)"
echo "docco $(docco --version)"

adduser --system hab || true
addgroup --system hab || true

sh /tmp/install.sh
hab install core/busybox-static
hab install core/hab-studio
hab --version
rm -rf /tmp/install.sh
