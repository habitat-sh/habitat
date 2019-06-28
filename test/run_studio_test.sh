#!/bin/bash

set -eou pipefail

studio_type=${1?studio type argument required}

sudo hab license accept
sudo hab pkg install core/expect
sudo hab pkg binlink core/expect expect 

pushd components/studio

test/"$studio_type"/test.sh
