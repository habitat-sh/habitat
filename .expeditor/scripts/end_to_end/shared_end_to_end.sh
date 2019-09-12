#!/bin/bash

set -euo pipefail

source .expeditor/scripts/shared.sh

### Placeholder for shared functions used by the end to end pipeline.

studio_run() {
  local studio_flags=""
  if [[ -n "${DOCKER_STUDIO_TEST:-}" ]]; then 
    studio_flags="-D"
  fi

  hab studio run "$studio_flags" "$@"
}

