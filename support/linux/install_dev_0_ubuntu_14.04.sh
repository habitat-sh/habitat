#!/bin/sh
set -eux

sudo -E apt-get update

sudo -E apt-get install -y --no-install-recommends \
  autotools-dev \
  autoconf \
  automake \
  build-essential \
  ca-certificates \
  cmake \
  curl \
  file \
  gdb \
  git \
  httpie \
  iproute2 \
  libpcre3-dev \
  libprotobuf-dev \
  libssl-dev \
  libtool \
  libunwind8-dev \
  man \
  musl-tools \
  npm \
  pkg-config \
  protobuf-compiler \
  software-properties-common \
  sudo \
  uuid-dev \
  vim \
  wget

# Install libsodium for zmq even though it will be automatically vendered with the sodiumoxide crate
(cd /tmp && git clone https://github.com/jedisct1/libsodium.git)
(cd /tmp/libsodium \
  && ./autogen.sh \
  && ./configure \
  && make \
  && sudo -E make install \
)
rm -rf /tmp/libsodium

(cd /tmp && git clone git://github.com/zeromq/libzmq.git)
(cd /tmp/libzmq \
  && ./autogen.sh \
  && ./configure --with-libsodium \
  && make \
  && sudo -E make install \
  && sudo -E ldconfig \
)
rm -rf /tmp/libzmq

(cd /tmp && git clone https://github.com/zeromq/czmq.git)
(cd /tmp/czmq \
  && ./autogen.sh \
  && ./configure \
  && make \
  && sudo -E make install \
  && sudo -E ldconfig \
)
rm -rf /tmp/czmq
