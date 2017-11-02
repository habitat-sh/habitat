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
if [ -f /etc/arch-release ]; then
  # According to https://docs.docker.com/engine/installation/linux/archlinux/
  # the Docker package is managed by the Arch Linux community
  sudo -E pacman -S --noconfirm docker
else
  if ! curl -sSL https://get.docker.io | sudo -E sh; then
    echo "Docker install from https://get.docker.io failed"
    if command -v lsb_release >/dev/null 2>&1; then
      echo "Check that this release version is still supported:"
      lsb_release -a
    fi
    exit 1
  fi
fi
docker --version

if [ ! -f /usr/bin/node ] && [ -f /usr/bin/nodejs ]; then
  sudo -E ln -snf /usr/bin/nodejs /usr/bin/node
fi
sudo -E npm install -g docco
echo "Node $(node --version)"
echo "npm $(npm --version)"
echo "docco $(docco --version)"

if [ ! -f /usr/local/bin/rq ]; then
  curl -sSLf https://sh.dflemstr.name/rq | bash -s -- --yes false
fi

if command -v useradd > /dev/null; then
  sudo -E useradd --system --no-create-home hab || true
else
  sudo -E adduser --system hab || true
fi
if command -v groupadd > /dev/null; then
  sudo -E groupadd --system hab || true
else
  sudo -E addgroup --system hab || true
fi

sudo -E sh /tmp/install.sh
sudo -E hab install core/busybox-static core/hab-studio
sudo -E rm -rf /tmp/install.sh
