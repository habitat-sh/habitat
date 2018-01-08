#!/bin/sh
set -eux

sudo -E apt-get update

sudo -E apt-get install -y --no-install-recommends \
  build-essential \
  ca-certificates \
  cmake \
  curl \
  direnv \
  file \
  gdb \
  git \
  httpie \
  iproute2 \
  libarchive-dev \
  libprotobuf-dev \
  libsodium-dev \
  libssl-dev \
  libczmq-dev \
  man \
  musl-tools \
  net-tools \
  pkg-config \
  postgresql-server-dev-9.6 \
  protobuf-compiler \
  software-properties-common \
  sudo \
  tmux \
  vim \
  wget
