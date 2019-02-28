#!/bin/bash

set -euo pipefail

d=$(dirname "${BASH_SOURCE[0]}")
source "$d/shared.sh"

install_rustfmt
