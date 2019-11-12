#!/bin/bash

set -eou pipefail

sudo hab license accept

# Studio test uses relative paths to copy necessary files and expects to be run
# from the studio source directory
pushd components/studio

test/test.sh
