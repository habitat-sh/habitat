#!/bin/bash

# Test that when SSL_CERT_FILE is an invalid certificate
# communications with builder still function in the studio

set -euo pipefail

source .expeditor/scripts/end_to_end/shared_end_to_end.sh

echo "--- Generating a signing key"
hab origin key generate "$HAB_ORIGIN"

echo "--- Generating invalid ssl certificate"
tempdir="$(mktemp --tmpdir --directory e2e-ssl-XXXXXX)"
e2e_certname="invalid-cert.pem"
echo "I AM NOT A CERTIFICATE" > "$tempdir/$e2e_certname"

export SSL_CERT_FILE="${tempdir}/${e2e_certname}"

echo "--- Test Builder communications with invalid SSL_CERT_FILE"
hab studio rm
studio_run echo "SSL_CERT_FILE: \$SSL_CERT_FILE"
studio_run hab pkg search core/vim

