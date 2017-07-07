#!/bin/bash

set -e

# Current version being deployed
version="$(cat VERSION)"
# Address of the mac builder
mac_builder=admin@74.80.245.236

BINTRAY_REPO=unstable
if [ $version == "$TRAVIS_TAG" ]; then
  BINTRAY_REPO=stable
fi

# kick off the mac unstable build
echo "Kicking off the $BINTRAY_REPO mac build"
var_file=/tmp/our-awesome-vars
ssh_args="-o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null -i /tmp/habitat-srv-admin"

if [ "${TRAVIS_PULL_REQUEST}" != "false" ]; then
  co="FETCH_HEAD"
  fetch="git fetch origin +refs/pull/$TRAVIS_PULL_REQUEST/merge:"
else
  co=$TRAVIS_COMMIT
  fetch=""
fi

# first update the copy of the habitat code stored on the mac server to the latest
ssh ${ssh_args} ${mac_builder} << EOF
    hab_src_dir="\$HOME/code/$TRAVIS_BUILD_NUMBER"
    mkdir -p \${hab_src_dir}
    cd \${hab_src_dir}
    git clone https://github.com/habitat-sh/habitat
    cd habitat
    eval $fetch
    git checkout -qf $co
    chmod 755 support/ci/deploy_mac.sh
EOF

# passing environment variables over ssh is a pain and never worked quite right.
# instead, write this out to a file and scp it over, to source later.
cat << EOF >${var_file}
export BINTRAY_REPO=$BINTRAY_REPO
export HAB_ORIGIN_KEY=$HAB_ORIGIN_KEY
export BINTRAY_USER=$BINTRAY_USER
export BINTRAY_KEY=$BINTRAY_KEY
export BINTRAY_PASSPHRASE=$BINTRAY_PASSPHRASE
export TRAVIS_BUILD_NUMBER=$TRAVIS_BUILD_NUMBER
EOF

scp ${ssh_args} ${var_file} ${mac_builder}:~/tmp
rm ${var_file}

# kick off the build
ssh ${ssh_args} ${mac_builder} \
  "sudo ~/code/$TRAVIS_BUILD_NUMBER/habitat/support/ci/deploy_mac.sh"
