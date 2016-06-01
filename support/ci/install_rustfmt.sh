#!/bin/bash
set -eu

version=0.5.0

if command -v rustfmt >/dev/null; then
  if [[ $(rustfmt --version) = "$version" ]]; then
    echo "--> Detected rustfmt version $version, skipping install"
    exit 0
  fi
fi

cargo install --vers $version rustfmt
