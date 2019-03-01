#!/bin/bash

set -euo pipefail

d=$(dirname "${BASH_SOURCE[0]}")
source "$d/shared.sh"

# Thanks to the magic of global variables, this value will get overwritten when
# install_rustfmt runs. But, if we don't declare it up front here, shellcheck
# errors on the cargo_fmt statement below.
toolchain="stable"

maybe_install_rustup
maybe_install_rustfmt
cargo_fmt="cargo +$toolchain fmt --all -- --check"
echo "--- :rust: Running cargo fmt command: $cargo_fmt"
$cargo_fmt
