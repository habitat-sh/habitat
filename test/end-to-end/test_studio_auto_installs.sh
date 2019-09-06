#!/bin/bash
# This test is designed to catch the regression described in 
# https://github.com/habitat-sh/habitat/issues/6771
# 
# When a user runs `hab studio enter` for the first time after installing a
# new Habitat release, the `core/hab-studio` package won't be present on the 
# system and the cli will automatically download and install the appropirate 
# package. Since we always install the studio as part of our build process to 
# ensure we're using the correct version, this behavior needs to be exercised 
# as its own test. 

set -euo pipefail

# Ensure there are no studios installed
while [ -d /hab/pkgs/core/hab-studio ]; do 
  hab pkg uninstall core/hab-studio
done

# 'studio enter' requires a signing key to be present for the current origin
echo "--- Generating signing key for $HAB_ORIGIN"
hab origin key generate "$HAB_ORIGIN" 

echo "--- Creating new studio"
hab studio new

echo "--- $(hab studio version)"
