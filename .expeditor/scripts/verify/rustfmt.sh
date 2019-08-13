#!/bin/bash

set -euo pipefail

source .expeditor/scripts/verify/shared.sh

toolchain=$(get_rustfmt_toolchain)

install_rustup
install_rustfmt "$toolchain"
cargo_fmt="cargo +$toolchain fmt --all -- --check"
echo "--- :rust: Running cargo fmt command: $cargo_fmt"
$cargo_fmt
