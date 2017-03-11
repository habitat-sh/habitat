#!/bin/bash

# fail fast if we aren't on the desired branch or if this is a pull request
if  [[ "${TRAVIS_BRANCH}" != "$(cat VERSION)" && ("${TRAVIS_PULL_REQUEST}" != "false" || "${TRAVIS_BRANCH}" != "master") ]]; then
    echo "We only publish on successful builds of master."
    exit 0
fi

# now do the linux unstable build
BOOTSTRAP_DIR=/root/travis_bootstrap
TEST_BIN_DIR=/root/hab_bins
TRAVIS_HAB=${BOOTSTRAP_DIR}/hab
HAB_DOWNLOAD_URL="https://api.bintray.com/content/habitat/stable/linux/x86_64/hab-%24latest-x86_64-linux.tar.gz?bt_package=hab-x86_64-linux"
export HAB_ORIGIN=core

BINTRAY_REPO=unstable
if [ "$(cat VERSION)" == "$TRAVIS_TAG" ]; then
  BINTRAY_REPO=stable
else
  export HAB_DEPOT_URL=http://app.acceptance.habitat.sh/v1/depot
fi

mkdir -p ${BOOTSTRAP_DIR}
# download a hab binary to build hab from source in a studio
wget -O hab.tar.gz "${HAB_DOWNLOAD_URL}"
# install it in a custom location
tar xvzf ./hab.tar.gz --strip 1 -C ${BOOTSTRAP_DIR}

# kick off the mac unstable build
echo "Kicking off the unstable mac build"
var_file=/tmp/our-awesome-vars
mac_builder=admin@74.80.245.236

# first update the copy of the habitat code stored on the mac server to the latest
ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null \
  -i /tmp/habitat-srv-admin ${mac_builder} \
  "~/bin/update_habitat_code.sh"

set -e

# passing environment variables over ssh is a pain and never worked quite right.
# instead, write this out to a file and scp it over, to source later.
cat << EOF >${var_file}
export BINTRAY_REPO=$BINTRAY_REPO
export HAB_ORIGIN_KEY=$HAB_ORIGIN_KEY
export BINTRAY_USER=$BINTRAY_USER
export BINTRAY_KEY=$BINTRAY_KEY
export BINTRAY_PASSPHRASE=$BINTRAY_PASSPHRASE
EOF

scp -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null \
  -i /tmp/habitat-srv-admin ${var_file} ${mac_builder}:~/tmp
rm ${var_file}

# kick off the build
ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null \
  -i /tmp/habitat-srv-admin ${mac_builder} \
  "sudo ~/code/habitat/support/ci/deploy_mac.sh"

# so key stuff doesn't get funky
unset SUDO_USER

# create our origin key
 cat << EOF > core.sig.key
SIG-SEC-1
core-20160810182414

${HAB_ORIGIN_KEY}
EOF

${TRAVIS_HAB} origin key import < ./core.sig.key
rm ./core.sig.key

COMPONENTS=($COMPONENTS)
for component in "${COMPONENTS[@]}"
do
  echo "Building $component"
  ${TRAVIS_HAB} studio run HAB_CARGO_TARGET_DIR=/src/target build components/${component}

  HART=$(find ./results -name *${component}*.hart)
  ${TRAVIS_HAB} pkg install $HART

  if [ -n "$HAB_AUTH_TOKEN" ]; then
    ${TRAVIS_HAB} pkg upload $HART
  fi

  # once we have built the stuio, switch over to bits built here
  if [[ "${component}" == "studio" ]]; then
    TRAVIS_HAB=$(find /hab/pkgs/core/hab -type f -name hab)
  elif [[ "${component}" == "hab" ]]; then
    RELEASE="${HART}_keep"
    cp $HART $RELEASE
  fi

  rm $HART
done

echo "Publishing hab to $BINTRAY_REPO"
${TRAVIS_HAB} pkg exec core/hab-bintray-publish publish-studio
${TRAVIS_HAB} pkg exec core/hab-bintray-publish publish-hab -r $BINTRAY_REPO $RELEASE
