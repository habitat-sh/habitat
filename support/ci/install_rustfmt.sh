#!/bin/bash
set -eu

version=0.8.4

if command -v rustfmt >/dev/null; then
  if [[ $(rustfmt --version | cut -d ' ' -f 1) = "$version" ]]; then
    echo "--> Detected rustfmt version $version, skipping install"
    exit 0
  fi
fi

echo "--> Removing rustfmt version $(rustfmt --version) and installing $version"
cargo uninstall rustfmt || true
cargo install --vers $version rustfmt
