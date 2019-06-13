#!/usr/bin/env bash

set -euo pipefail 

export HAB_LICENSE="accept-no-persist"

hab studio rm

studio_command="hab studio"

./test/shared/test-all.sh "${studio_command}"

