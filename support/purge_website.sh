#!/bin/bash

set -e

curl -H "Fastly-Key: ${FASTLY_API_KEY}" -X POST "https://api.fastly.com/service/${FASTLY_SERVICE_KEY}/purge_all"
