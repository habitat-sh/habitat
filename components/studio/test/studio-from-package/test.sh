#!/usr/bin/env bash

set -euo pipefail 

export HAB_LICENSE="accept-no-persist"

hab studio rm

export STUDIO_COMMAND="hab studio"

./test/shared/test-all.sh

