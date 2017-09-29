#!/bin/bash

export SSL_CERT_FILE
SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt

/bin/hab term
sleep 1
exec /bin/hab "$@"
