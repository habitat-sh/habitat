#!/bin/bash

set -euo pipefail

d=$(dirname "${BASH_SOURCE[0]}")
source "$d/shared.sh"

maybe_install_rustup
install_rustfmt
cargo_fmt="cargo +$toolchain fmt --all -- --check"
echo "--- :rust: Running cargo fmt command: $cargo_fmt"
$cargo_fmt
