#!/bin/bash

set -euo pipefail

dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"
# shellcheck disable=SC1090
source "$dir/shared.sh"
toolchain=$(get_nightly_toolchain)

install_rustup
install_rustfmt "$toolchain"
cargo_fmt="cargo +$toolchain fmt --all -- --check"
echo "--- :rust: Running cargo fmt command: $cargo_fmt"
$cargo_fmt
