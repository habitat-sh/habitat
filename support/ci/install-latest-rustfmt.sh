#!/bin/bash

set -euo pipefail

d=$(dirname "${BASH_SOURCE[0]}")
source "$d/shared.sh"

maybe_install_rustfmt
