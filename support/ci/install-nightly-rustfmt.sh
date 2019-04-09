#!/bin/bash

set -euo pipefail

dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"
# shellcheck disable=SC1090
source "$dir/shared.sh"
toolchain=$(get_nightly_toolchain)

install_rustfmt "$toolchain"
