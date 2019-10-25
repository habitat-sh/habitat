#!/bin/bash

# This tests that a self-signed certificate in the cache is loaded 
# by hab. We don't have a way to inspect loaded certificates at this
# time, so we rely on inspecting debug output which is less than ideal.
# As we improve our pipeline, this test could be replaced by attemting 
# to talk to a builder service with a self-signed cert.
 
echo "--- Generating self-signed ssl certificate"
mkdir -p /hab/cache/ssl
e2e_cert="/hab/cache/ssl/e2e-ssl.pem"
openssl req -newkey rsa:2048 -batch -nodes -keyout key.pem -x509 -days 365 -out "${e2e_cert}"

echo "--- Inspecting debug output for loaded certificate"
env RUST_LOG=debug hab pkg search core/redis 2>&1 | grep "Processing cert file: ${e2e_cert}"
