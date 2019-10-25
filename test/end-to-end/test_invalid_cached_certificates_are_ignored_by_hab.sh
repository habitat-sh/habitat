#!/bin/bash

# Test that when an invalid certificate is placed in the cache directory 
# `hab` ignores that certificate. Since we don't have a way to verify a cert
# is ignored aside from inspecting debug output, we test that we can still
# install a package even when a bad cert is cached.

set -euo pipefail

source .expeditor/scripts/end_to_end/shared_end_to_end.sh

echo "--- Generating a signing key"
hab origin key generate "$HAB_ORIGIN"

echo "--- Generating self-signed ssl certificate"
e2e_certname="invalid-ssl-cert.pem"
mkdir -p /hab/cache/ssl
echo "I AM NOT A CERTIFICATE" > "/hab/cache/ssl/${e2e_certname}"

echo "--- Test Builder communications with invalid cert"
# Specify the stable channel, since our environment is using DEV to 
# install `hab` packages.
hab pkg install core/redis --channel stable
