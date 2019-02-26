#!/bin/bash

set -euo pipefail

# This script can be run standalone, but it can also be sourced from rustfmt.sh.
# If it's sourced, then $rustup is already defined for us. If it's not, we need
# to make sure we have it.
: "${rustup:=$(command -v rustup)}"

# Due to the nature of nightly rust, sometimes changes will break rustfmt's
# usage of rustc. If this happens, nightly rust won't include rustfmt,
# and we need to automatically fall back to a version that does include it.
# Note that we begin with 1 day ago, since nightly packages can sometimes not
# exist when this script runs if we leave it at 0.
max_days=90

for days_ago in $(seq 1 1 $max_days)
do
  date=$(date -d "$days_ago days ago" +%Y-%m-%d)
  toolchain="nightly-$date"
  echo "Installing rust $toolchain"
  sudo -E "$rustup" toolchain install "$toolchain"

  if sudo -E "$rustup" component add --toolchain "$toolchain" rustfmt; then
    if [ "${BASH_SOURCE[0]}" != "${0}" ]; then
      # If we made it here, this script is being sourced from rustfmt.sh, so do the check
      cargo_fmt="$rustup run $toolchain cargo fmt --all -- --check"
      echo "Running cargo fmt command: $cargo_fmt"
      $cargo_fmt
    fi

    exit
  else
    next_days=$((days_ago + 1))
    echo "Rust $toolchain did not include rustfmt. Let's try $next_days day(s) ago."
  fi
done

echo "We couldn't find a release of nightly rust in the past $max_days days that includes rustfmt. Giving up entirely."
exit 1
