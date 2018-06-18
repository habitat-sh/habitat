#!/bin/bash

set -e

# now do the linux unstable build
HAB_VERSION=$(cat VERSION)
BOOTSTRAP_DIR=/root/travis_bootstrap
TEST_BIN_DIR=/root/hab_bins
TRAVIS_HAB=${BOOTSTRAP_DIR}/hab
HAB_DOWNLOAD_URL="https://api.bintray.com/content/habitat/stable/linux/x86_64/hab-%24latest-x86_64-linux.tar.gz?bt_package=hab-x86_64-linux"

export HAB_ORIGIN=core

if [ "${HAB_VERSION}" == "${TRAVIS_TAG}" ]; then
    IS_RELEASE_BUILD=1
fi

if [ -n "${IS_RELEASE_BUILD}" ]; then
    # We'll upload to the stable repo, but not *publish* until we've
    # validated it
    BINTRAY_REPO=stable

    # Similarly, we'll promote Habitat packages to a release-specific
    # channel; once we've validated them outside of Travis, we'll
    # promote to stable
    export CI_OVERRIDE_CHANNEL="rc-${HAB_VERSION}"
else
    # Not a release build, just a normal build from the master
    # branch. It's all unstable, all the time.
    BINTRAY_REPO=unstable
    export CI_OVERRIDE_CHANNEL=unstable
fi

mkdir -p ${BOOTSTRAP_DIR}
# download a hab binary to build hab from source in a studio
wget -O hab.tar.gz "${HAB_DOWNLOAD_URL}"
# install it in a custom location
tar xvzf ./hab.tar.gz --strip 1 -C ${BOOTSTRAP_DIR}
rm hab.tar.gz

# so key stuff doesn't get funky
unset SUDO_USER

# create our origin key
cat << EOF > core.sig.key
SIG-SEC-1
core-20160810182414

${HAB_ORIGIN_KEY}
EOF

if [ -n "$HAB_ORIGIN_KEY" ]; then
  ${TRAVIS_HAB} origin key import < ./core.sig.key
fi

rm ./core.sig.key

PACKAGES=($PACKAGES)
for package in "${PACKAGES[@]}"
do
  # Always build the package if it's a release; otherwise, only build a
  # package if there are relevant code changes.
  if [ -n "${IS_RELEASE_BUILD}" ] || ./support/ci/should_we_build.sh "${package}" ; then

     echo "Building ${package}..."

     echo "--> Clearing any pre-exisiting $HAB_ORIGIN secret keys from the Studio"
     env HAB_ORIGIN= ${TRAVIS_HAB} studio run sh -c \'rm -f /hab/cache/keys/*-*.sig.key\'

     # The names of the Habitat packages we create are often different
     # from the directories in which they reside; specifically, we
     # often add the prefix "hab-" to the name of the package, but
     # leave this off of the directory for ease of navigation. Here,
     # we normalize this.
     if [ -d "components/${package}" ]; then
         directory_name=${package}
     else
         truncated_name=${package##hab-}
         if [ -d "components/${truncated_name}" ] ; then
             directory_name=${truncated_name}
         else
             echo "Cannot find code directory for package '${package}' after looking in the following locations:"
             echo
             echo "    components/${package}"
             echo "    components/${truncated_name}"
             echo
             exit 1
         fi
     fi

     ${TRAVIS_HAB} studio run HAB_CARGO_TARGET_DIR=/src/target build components/${directory_name}

     source ./results/last_build.env
     HART="./results/$pkg_artifact"
     ${TRAVIS_HAB} pkg install $HART

     # Once we have built the studio, switch over to bits built here
     if [[ "${package}" == "hab-studio" ]]; then
         # Marker variable that indicates we have a studio artifact to upload.
         PUBLISH_HAB_STUDIO=true
         # We switch the Hab binary we use here, rather than when we
         # build the hab package, because the version of hab
         # determines the version of the studio we use. Thus, we have
         # to wait to make the switch until there is a studio artifact
         # for us to use.
         TRAVIS_HAB=$(find /hab/pkgs/core/hab -type f -name hab | tail -1)
     elif [[ "${package}" == "hab" ]]; then
         HAB_RELEASE_ARTIFACT="${HART}_keep"
         cp $HART $HAB_RELEASE_ARTIFACT
     fi

     if [ -n "$HAB_AUTH_TOKEN" ]; then
         ${TRAVIS_HAB} pkg upload $HART --channel $CI_OVERRIDE_CHANNEL
     fi

     rm $HART
  else
     echo "Skipping build of ${package}; no relevant code changes"
  fi
done

echo "--> Removing origin secret keys from Studio"
env HAB_ORIGIN= ${TRAVIS_HAB} studio run sh -c \'rm -f /hab/cache/keys/*-*.sig.key\'

if [ -n "$BINTRAY_USER" ]; then
  if [ ! -d "/hab/pkgs/core/hab-bintray-publish" ] ; then
      # If we didn't build core/hab-bintray-publish and subsequently
      # install it on this run (which is possible on master branch
      # builds), let's ensure that we can still publish things to
      # Bintray, shall we?
      echo "Installing core/hab-bintray-publish from Builder..."
      ${TRAVIS_HAB} pkg install core/hab-bintray-publish
  fi

  # On 'master' branch builds, we might not build hab-studio. Only
  # publish if we do.
  if [ -n "${PUBLISH_HAB_STUDIO}" ]; then
      echo "Publishing hab-studio to Bintray"
      env HAB_BLDR_CHANNEL=$CI_OVERRIDE_CHANNEL ${TRAVIS_HAB} pkg exec core/hab-bintray-publish publish-studio
  fi

  # On 'master' branch builds, we might not build a hab artifact to
  # publish. It'll always be there on release builds, though.
  if [ -n "${HAB_RELEASE_ARTIFACT}" ]; then
      echo "Publishing hab to $BINTRAY_REPO"
      if [ -n "${IS_RELEASE_BUILD}" ]; then
          env HAB_BLDR_CHANNEL=$CI_OVERRIDE_CHANNEL ${TRAVIS_HAB} pkg exec core/hab-bintray-publish publish-hab -s -r $BINTRAY_REPO $HAB_RELEASE_ARTIFACT
      else
          env HAB_BLDR_CHANNEL=$CI_OVERRIDE_CHANNEL ${TRAVIS_HAB} pkg exec core/hab-bintray-publish publish-hab -r $BINTRAY_REPO $HAB_RELEASE_ARTIFACT
      fi
  fi
fi
