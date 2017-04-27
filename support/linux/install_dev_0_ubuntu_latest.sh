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
