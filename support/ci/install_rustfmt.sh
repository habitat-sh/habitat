#!/bin/bash
set -eu

version=0.3.4

if command -v rustfmt >/dev/null; then
  if [[ $(cargo +nightly fmt -- --version | cut -d ' ' -f 1) = "$version-nightly" ]]; then
    echo "--> Detected rustfmt version $version, skipping install"
    exit 0
  fi
fi

echo "--> Removing rustfmt version $(cargo +nightly fmt -- --version) and installing $version"
cargo uninstall rustfmt || true
cargo uninstall rustfmt-nightly || true
cargo +nightly install --vers $version --force rustfmt-nightly
