#!/bin/bash

# TODO: These are being temporarily disabled while the build is in flux for the
# new version of the supervisor.

exit 0

# This script builds hab and hab-sup from source via a Habitat studio.
# The binaries are then extracted and copied to /root/hab_bins
# where rpsec uses them during testing. We do this to ensure that
# hab and hab-sup are built with the correct dependencies without
# linking any deps that are available on Travis.

#  BOOTSTRAP_DIR=/root/travis_bootstrap
#  TEST_BIN_DIR=/root/hab_bins
#  HAB_DOWNLOAD_URL="https://api.bintray.com/content/habitat/stable/linux/x86_64/hab-%24latest-x86_64-linux.tar.gz?bt_package=hab-x86_64-linux"
#  TRAVIS_HAB=${BOOTSTRAP_DIR}/hab
#  export HAB_ORIGIN=hab_travis
#  
#  
#  git log HEAD~1..HEAD | grep -q '!!! Temporary Commit !!!'
#  is_tmp_commit=$?
#  
#  # When we're on a temporary commit, don't do anything.
#  if [[ $is_tmp_commit = 0 ]]; then
#    exit 0
#  fi
#  
#  # if Hab detects that we're using sudo, it won't find our generated keys
#  unset SUDO_USER
#  
#  run_tests() {
#    cd test || exit
#    ./test.sh
#  }
#  
#  # periodically output a char so Travis doesn't timeout on us
#  function periodic_output() {
#    while true; do
#      sleep 360
#      printf .
#    done
#  }
#  
#  # building hab and hab-sup in a studio can take > 10 minutes, so we call this
#  # function to prevent Travis from timing out on us.
#  periodic_output &
#  
#  mkdir -p ${BOOTSTRAP_DIR}
#  
#  # make sure it's clean!
#  rm -rf ${TEST_BIN_DIR}
#  mkdir -p ${TEST_BIN_DIR}
#  mkdir -p /hab/cache/keys
#  
#  # download a hab binary to build hab from source in a studio
#  wget -O hab.tar.gz "${HAB_DOWNLOAD_URL}"
#  # install it in a custom location
#  tar xvzf ./hab.tar.gz --strip 1 -C ${BOOTSTRAP_DIR}
#  
#  # generate a key to use to build `hab` and `hab-sup` in a studio
#  # we won't need this after we extract the binaries later in the script.
#  # They don't need to be stored past a single Travis run.
#  ${TRAVIS_HAB} origin key generate hab_travis
#  
#  echo "Building hab"
#  ${TRAVIS_HAB} studio build components/hab > /root/hab_build.log 2>&1
#  
#  echo "Building hab-sup"
#  ${TRAVIS_HAB} studio build components/sup > /root/hab_sup_build.log 2>&1
#  
#  echo "Installing hab and hab-sup"
#  # install the artifacts
#  ${TRAVIS_HAB} pkg install ./results/*.hart
#  
#  # copy the binaries out built packages
#  # this will most likely fail if you have compiled hab + sup more than once,
#  # hence the rm -rf ${TEST_BIN_DIR} above
#  find /hab/pkgs/hab_travis/hab/ -type f -name hab -exec cp {} ${TEST_BIN_DIR} \;
#  find /hab/pkgs/hab_travis/hab-sup/ -type f -name hab-sup -exec cp {} ${TEST_BIN_DIR} \;
#  
#  
#  # TODO
#  # https://docs.travis-ci.com/user/pull-requests
#  if [ "${TRAVIS_PULL_REQUEST}" = "false" ]; then
#      run_tests
#  else
#      run_tests
#  fi
