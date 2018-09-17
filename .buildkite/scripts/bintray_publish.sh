#!/bin/bash

set -euo pipefail

source .buildkite/scripts/shared.sh

if is_fake_release; then
    bintray_repository=unstable
else
    bintray_repository=stable
fi

echo "--- Preparing to publish artifacts to the ${bintray_repository} Bintray repository"

publish() {
    local target version release repository url

    target=${1}
    version=${2}
    release=${3}
    repository=${4}
    url="https://api.bintray.com/content/habitat/${repository}/${target}/${version}-${release}/publish"
    if is_fake_release; then
        echo "--- :warning: If this were a real release, we would have hit ${url}"
    else
        curl -u "${BINTRAY_USER}:${BINTRAY_KEY}" -X POST "${url}"
    fi
}

echo "--- :habicat: Publishing all Habitat CLI artifacts in Bintray"

version=$(get_version)

########################################################################
# Linux x86-64-linux Publish
release=$(get_hab_release x86_64-linux)
echo "--- :linux: Publishing x86-64-linux 'hab' ${version}-${release} on Bintray"
publish "hab-x86_64-linux" "${version}" "${release}" "${bintray_repository}"

########################################################################
# Linux x86-64-linux-kernel2 Publish
release=$(get_hab_release x86_64-linux-kernel2)
echo "--- :linux: :two: Publishing x86-64-linux-kernel2 'hab' ${version}-${release} on Bintray"
publish "hab-x86_64-linux-kernel2" "${version}" "${release}" "${bintray_repository}"

########################################################################
# macOS Publish
release=$(get_hab_release x86_64-darwin)
echo "--- :mac: Publishing x86-64-darwin 'hab' ${version}-${release} to Bintray"
publish "hab-x86_64-darwin" "${version}" "${release}" "${bintray_repository}"

########################################################################
# Windows Publish
release=$(get_hab_release x86_64-windows)
echo "--- :windows: Publishing x86-64-windows 'hab' ${version}-${release} to Bintray"
publish "hab-x86_64-windows" "${version}" "${release}" "${bintray_repository}"
