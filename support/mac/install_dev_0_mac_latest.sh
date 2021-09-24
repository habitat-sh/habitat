#!/bin/sh
set -eux

if ! command -v brew >/dev/null; then
  echo "Homebrew missing, attempting to install"
  /usr/bin/ruby -e "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/main/install)" < /dev/null
fi

brew update

brew install \
  node \
  pkg-config \
  zeromq
