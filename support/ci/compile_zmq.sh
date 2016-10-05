#!/bin/bash
set -eu

version=4.1.4
nv=zeromq-$version
source=http://download.zeromq.org/zeromq-$version.tar.gz
prefix=$HOME/pkgs/zeromq/$version

# If installed, exit early!
if [ -d "$prefix" ]; then
  echo "--> Detected $nv installed under $prefix, skipping compile"
  exit 0
fi

echo "--> Compiling $nv"
trap 'rm -rf /tmp/${nv}*; exit $?' INT TERM EXIT
(cd /tmp && wget --no-check-certificate $source && tar xf /tmp/$(basename $source))
cd /tmp/$nv
./autogen.sh && ./configure --prefix=$prefix --with-libsodium
make
make install
