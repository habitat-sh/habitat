#!/bin/bash

set -eou pipefail

source .expeditor/scripts/shared.sh

curlbash_hab "x86_64-linux"

sudo -E hab license accept

# Studio test uses relative paths to copy necessary files and expects to be run
# from the studio source directory
pushd components/studio

test/test.sh
