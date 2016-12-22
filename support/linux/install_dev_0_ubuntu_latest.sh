#!/bin/sh
set -eux

sudo -E apt-get update

sudo -E apt-get install -y --no-install-recommends \
  build-essential \
  ca-certificates \
  cmake \
  curl \
  file \
  gdb \
  iproute2 \
  libarchive-dev \
  libprotobuf-dev \
  libsodium-dev \
  libssl-dev \
  libczmq-dev \
  man \
  musl-tools \
  net-tools \
  npm \
  pkg-config \
  protobuf-compiler \
  redis-server \
  software-properties-common \
  sudo \
  tmux \
  vim \
  wget

# Let's get us a shiny new 7.12+ version of GDB to get the latest Rust support
(gdb_pkg='http://launchpadlibrarian.net/289215234/gdb_7.12-0ubuntu1_amd64.deb' \
  && cd /tmp \
  && wget "$gdb_pkg" \
  && sudo dpkg -i $(basename $gdb_pkg) \
  && rm -f $(basename $gdb_pkg) \
)
