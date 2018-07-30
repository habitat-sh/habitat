#!/bin/bash

set -euo pipefail

source .buildkite/scripts/shared.sh

if [[ "${FAKE_RELEASE_TAG}" ]]; then
    echo "--- :warning: FAKE_RELEASE_TAG was found in the environment! This isn't a \"real\" release!"
    fake_release=1
fi

publish(){
    url=${1}
    if [ -z "${fake_release}" ]; then
        curl -u "${BINTRAY_USER}:${BINTRAY_KEY}" -X POST "${url}"
    else
        echo "--- :warning: If this were a real release, we would have hit ${url}"
    fi
}

PROMOTE_CHANNEL="stable"
echo "--- :habicat: Publishing all Habitat CLI artifacts in Bintray"

version=$(buildkite-agent meta-data get "version")

########################################################################
# Linux Publish
release=$(buildkite-agent meta-data get "hab-release-linux")
echo "--- :linux: Publishing Linux 'hab' ${version}-${release} on Bintray"
publish "https://api.bintray.com/content/habitat/$PROMOTE_CHANNEL/hab-x86_64-linux/${version}-${release}/publish"

########################################################################
# MacOS Publish

release=$(buildkite-agent meta-data get "hab-release-macos")
echo "--- :mac: Publishing MacOS 'hab' ${version}-${release} to Bintray"
publish "https://api.bintray.com/content/habitat/$PROMOTE_CHANNEL/hab-x86_64-darwin/${version}-${release}/publish"

########################################################################
# Windows Publish
#
# NOTE: Windows releases aren't yet built in Buildkite, so we have to
# ask Builder what the release actually is... Appveyor puts this here
# for us.
channel=$(buildkite-agent meta-data get "release-channel")
windows_ident=$(latest_from_builder x86_64-windows "${channel}" hab "${version}")
release=$(echo "${windows_ident}" | awk 'BEGIN { FS = "/" } ; { print $4 }')
echo "--- :windows: Publishing Windows 'hab' ${version}-${release}"
publish "https://api.bintray.com/content/habitat/$PROMOTE_CHANNEL/hab-x86_64-windows/${version}-${release}/publish"
