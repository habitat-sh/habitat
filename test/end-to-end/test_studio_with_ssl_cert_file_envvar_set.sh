#!/bin/bash

# Test that SSL_CERT_FILE is persisted into the studio, set to 
# the correct internal path and that we can communicate with Builder

set -euo pipefail

source .expeditor/scripts/end_to_end/shared_end_to_end.sh

echo "--- Generating a signing key"
hab origin key generate "$HAB_ORIGIN"

echo "--- Generating self-signed ssl certificate"
tempdir="$(mktemp --tmpdir --directory e2e-ssl-XXXXXX)"
e2e_certname="e2e-ssl.pem"
openssl req -newkey rsa:2048 -batch -nodes -keyout key.pem -x509 -days 365 -out "${tempdir}/${e2e_certname}"

export SSL_CERT_FILE="${tempdir}/${e2e_certname}"

echo "--- SSL_CERT_FILE is correctly set inside the studio: ${SSL_CERT_FILE})"
# If this test fails with `test: ==: unary operator expected`
# that likely indicates that $SSL_CERT_FILE was not passed into 
# the studio and is unset.
studio_run test "\$SSL_CERT_FILE" == "/hab/cache/ssl/${e2e_certname}"

echo "--- SSL_CERT_FILE is copied into the studio"
studio_run test -f "/hab/cache/ssl/${e2e_certname}"

echo "--- Test Builder communications with self-signed cert"
studio_run hab pkg search core/vim

