#!/bin/bash

BOOTSTRAP_DIR=/root/travis_bootstrap
TEST_BIN_DIR=/root/hab_bins
TRAVIS_HAB=${BOOTSTRAP_DIR}/hab
HAB_DOWNLOAD_URL="https://api.bintray.com/content/habitat/stable/linux/x86_64/hab-%24latest-x86_64-linux.tar.gz?bt_package=hab-x86_64-linux"
export HAB_ORIGIN=core


# download a hab binary to build hab from source in a studio
wget -O hab.tar.gz "${HAB_DOWNLOAD_URL}"
# install it in a custom location
tar xvzf ./hab.tar.gz --strip 1 -C ${BOOTSTRAP_DIR}

unset SUDO_USER

cd ..
cat << EOF > core.sig.key
SIG-SEC-1
core-20160810182414

${HAB_ORIGIN_KEY}
EOF

${TRAVIS_HAB} origin key import < ./core.sig.key
rm ./core.sig.key

if ([ "${TRAVIS_PULL_REQUEST}" = "false" ] && ["${TRAVIS_BRANCH}" = "master" ]); then
    mkdir -p ./release
    rm -rf ./release/*
    echo "Building bintray-publish"
    ${TRAVIS_HAB} studio build habitat/components/bintray-publish > /root/bintray-publish_build.log 2>&1
    echo "Building hab"
    ${TRAVIS_HAB} studio build habitat/components/hab > /root/hab_build.log 2>&1
    echo "Built new unstable version of hab"
    PUBLISH=$(find ./results -name core-hab-bintray*.hart)
    RELEASE=$(find ./results -name core-hab-0*.hart)
    echo "Publishing hab to unstable"
    ${TRAVIS_HAB} pkg install $PUBLISH
    ${TRAVIS_HAB} pkg exec core/hab-bintray-publish publish-hab -r unstable $RELEASE
fi
