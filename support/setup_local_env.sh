#!/bin/bash

# set -euo pipefail

export HAB_LICENSE="accept-no-persist"
sudo -E hab pkg install core/bzip2
sudo -E hab pkg install core/libarchive
sudo -E hab pkg install core/libsodium
sudo -E hab pkg install core/openssl
sudo -E hab pkg install core/xz
sudo -E hab pkg install core/zlib
sudo -E hab pkg install core/zeromq
sudo -E hab pkg install core/protobuf --binlink
sudo -E hab pkg install core/rust --binlink

# for the libarchive crate
export LIBARCHIVE_LIB_DIR
LIBARCHIVE_LIB_DIR="$(hab pkg path core/libarchive)/lib"
export LIBARCHIVE_INCLUDE_DIR
LIBARCHIVE_INCLUDE_DIR="$(hab pkg path core/libarchive)/include"
# for the openssl crate
export OPENSSL_LIB_DIR
OPENSSL_LIB_DIR="$(hab pkg path core/openssl)/lib"
export OPENSSL_INCLUDE_DIR
OPENSSL_INCLUDE_DIR="$(hab pkg path core/openssl)/include"
# for the libsodium crate
export SODIUM_LIB_DIR
SODIUM_LIB_DIR="$(hab pkg path core/libsodium)/lib"
# so zmq knows where it lives
export LIBZMQ_PREFIX
LIBZMQ_PREFIX=$(hab pkg path core/zeromq)
# the libarchive and zmq look to the load lib path (as well as the lib path) for these
# we get this for free when running hab pkg build
export LD_LIBRARY_PATH
LD_LIBRARY_PATH="$(hab pkg path core/zlib)/lib:$(hab pkg path core/bzip2)/lib:$(hab pkg path core/xz)/lib:$(hab pkg path core/libarchive)/lib:$(hab pkg path core/libsodium)/lib:$(hab pkg path core/openssl)/lib:$(hab pkg path core/gcc)/lib:$LIBZMQ_PREFIX/lib"
# again we get this for free when running hab pkg build, here we have to set it so the lib path is set for cargo
export LIBRARY_PATH
LIBRARY_PATH=$LD_LIBRARY_PATH
export PKG_CONFIG_PATH
PKG_CONFIG_PATH="$(hab pkg path core/libarchive)/lib/pkgconfig:$(hab pkg path core/libsodium)/lib/pkgconfig:$(hab pkg path core/openssl)/lib/pkgconfig"
