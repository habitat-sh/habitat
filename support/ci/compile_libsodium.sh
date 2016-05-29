#!/bin/bash
set -eu

version=1.0.8
nv=libsodium-$version
source=https://download.libsodium.org/libsodium/releases/${nv}.tar.gz
prefix=$HOME/pkgs/libsodium/$version

# If installed, exit early!
if [ -d "$prefix" ]; then
  echo "--> Detected $nv installed under $prefix, skipping compile"
  exit 0
fi

echo "--> Compiling $nv"
trap 'rm -rf /tmp/${nv}*; exit $?' INT TERM EXIT
(cd /tmp && wget $source && tar xf /tmp/$(basename $source))
cd /tmp/$nv
./configure --prefix=$prefix
make
make install
