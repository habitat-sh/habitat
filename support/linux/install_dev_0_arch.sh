#!/bin/sh
set -eux

sudo -E pacman -Syyu --noconfirm

sudo -E pacman -S --noconfirm \
  base-devel \
  cmake \
  curl \
  gdb \
  git \
  man \
  npm \
  openssl \
  pkg-config \
  protobuf \
  redis \
  wget \
  zeromq
