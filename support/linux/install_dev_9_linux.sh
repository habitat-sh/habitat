#!/bin/sh
set -eux

# Install Rust and musl libc target
curl -sSf https://sh.rustup.rs \
  | env -u CARGO_HOME sh -s -- -y --default-toolchain stable
. $HOME/.cargo/env
env -u CARGO_HOME rustup target add x86_64-unknown-linux-musl
env -u CARGO_HOME cargo install protobuf
rustc --version
cargo --version

# Install Docker
if [ -f /etc/lsb-release ] \
    && [ "$(. /etc/lsb-release; echo $DISTRIB_DESCRIPTION)" = "Ubuntu 16.10" ]; then
  # Until there is a 1.13 release, there is no stable Docker package for Yakkety :/
  curl -sSL https://test.docker.com | sudo -E sh
else
  curl -sSL https://get.docker.io | sudo -E sh
fi
docker --version

if [ ! -f /usr/bin/node ] && [ -f /usr/bin/nodejs ]; then
  sudo -E ln -snf /usr/bin/nodejs /usr/bin/node
fi
sudo -E npm install -g docco
echo "Node $(node --version)"
echo "npm $(npm --version)"
echo "docco $(docco --version)"

sudo -E adduser --system hab || true
sudo -E addgroup --system hab || true

sudo -E sh /tmp/install.sh
sudo -E hab install core/busybox-static core/hab-studio
sudo -E rm -rf /tmp/install.sh
