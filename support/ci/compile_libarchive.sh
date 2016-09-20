#!/bin/bash
set -eu

version=3.2.0
nv=libarchive-$version
source=http://www.libarchive.org/downloads/${nv}.tar.gz
prefix=$HOME/pkgs/libarchive/$version
echo "LIBARCHIVE PREFIX = ${prefix}"

# If installed, exit early!
if [ -d "$prefix" ]; then
  echo "--> Detected $nv installed under $prefix, skipping compile"
  exit 0
fi

echo "--> Compiling $nv"
trap 'rm -rf /tmp/${nv}*; exit $?' INT TERM EXIT
(cd /tmp && wget $source && tar xf /tmp/$(basename $source))
cd /tmp/$nv
./configure --prefix=$prefix --without-xml2 --without-lzo2
make
make install
