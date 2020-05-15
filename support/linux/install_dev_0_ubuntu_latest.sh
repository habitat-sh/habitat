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
  libprotobuf-dev \
  libssl-dev \
  libczmq-dev \
  man \
  musl-tools \
  net-tools \
  pkg-config \
  libpq-dev \
  protobuf-compiler \
  software-properties-common \
  sudo \
  tmux \
  vim \
  wget
