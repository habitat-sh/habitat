#!/usr/bin/env bash

set -euo pipefail 

export HAB_LICENSE="accept-no-persist"

hab studio rm

export STUDIO_ENTER_COMMAND="hab studio enter"

./test/shared/test-all.sh

