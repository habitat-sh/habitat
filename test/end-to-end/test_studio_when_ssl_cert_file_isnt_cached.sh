#!/bin/bash

# Test that when SSL_CERT_FILE isn't cached, the variable isn't 
# set on the inside of the studio.  
# https://github.com/habitat-sh/habitat/issues/7219

set -euo pipefail

source .expeditor/scripts/end_to_end/shared_end_to_end.sh
 
echo "--- Generating a signing key"
hab origin key generate "$HAB_ORIGIN"

echo "--- Removing any existing cached certificates"
rm -f /hab/cache/ssl/*
rm -f ~/.hab/cache/ssl/*

# This must be run immediatly before `studio run`, as 
# the auto-installation of the studio on first-run triggers
# the behavior we're attempting to test
echo "--- Uninstalling any existing studio packages"
hab pkg uninstall core/hab-studio || true

echo "--- Test SSL_CERT_FILE remains unset inside the studio"
hab studio run "echo \$SSL_CERT_FILE && test ! -v SSL_CERT_FILE"

