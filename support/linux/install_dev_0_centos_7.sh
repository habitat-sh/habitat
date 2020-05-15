#!/bin/sh
set -eux

# Install the zeromq yum repo
curl http://download.opensuse.org/repositories/home:/fengshuo:/zeromq/CentOS_CentOS-6/home:fengshuo:zeromq.repo | sudo -E tee /etc/yum.repos.d/zeromq.repo

# Install the node.js yum repo
curl -sL https://rpm.nodesource.com/setup_6.x | sudo -E bash

# Install common development tools
sudo -E yum groupinstall -y 'Development Tools'

sudo -E yum install -y \
  git \
  gpm-libs \
  nodejs \
  openssl-devel \
  protobuf-devel \
  sudo \
  wget \
  which \
  zeromq-devel

# pkg-config will be able to find libsodium with the following:
export PKG_CONFIG_PATH=/usr/local/lib/pkgconfig
# needed for the Habitat binaries to find libsodium at runtime
export LD_LIBRARY_PATH=/usr/local/lib

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
