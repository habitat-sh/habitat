#!/bin/bash

# Test that an ssl certificate placed in the the users hab/cached/ssl directory
# is persisted into the studio, and that we can communicate with Builder

set -euo pipefail

source .expeditor/scripts/end_to_end/shared_end_to_end.sh

echo "--- Generating a signing key"
hab origin key generate "$HAB_ORIGIN"

echo "--- Generating self-signed ssl certificate"
tempdir="$(mktemp --tmpdir --directory e2e-ssl-XXXXXX)"
e2e_certname="custom-ssl-cert.pem"
openssl req -newkey rsa:2048 -batch -nodes -keyout "${tempdir}/key.pem" -x509 -days 365 -out "${tempdir}/${e2e_certname}"

# This test is assumed to run as root
mkdir -p /hab/cache/ssl
cp "${tempdir}/${e2e_certname}" /hab/cache/ssl/

echo "--- Certificate is present in studio ssl cache" 
studio_run test -f "/hab/cache/ssl/${e2e_certname}"

