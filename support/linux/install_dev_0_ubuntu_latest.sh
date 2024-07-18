#!/bin/sh
set -eux

DEBIAN_FRONTEND=noninteractive sudo -E apt-get install -yq --no-install-recommends \
  build-essential \
  ca-certificates \
  cmake \
  curl \
  direnv \
  file \
  git \
  httpie \
  iproute2 \
  libprotobuf-dev \
  libssl-dev \
  libczmq-dev \
  musl-tools \
  net-tools \
  pkg-config \
  libpq-dev \
  protobuf-compiler \
  software-properties-common \
  sudo \
  tmux \
  wget
