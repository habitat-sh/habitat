#!/bin/bash

set -euo pipefail

channel=$(buildkite-agent meta-data get "release-channel")

echo "--- :thinking_face: Determining which channel to promote to"
if [[ "${FAKE_RELEASE_TAG}" ]]; then
    echo "FAKE_RELEASE_TAG was found in the environment! This isn't a \"real\" release!"
    PROMOTE_CHANNEL="fakestable-${FAKE_RELEASE_TAG}"
else
    PROMOTE_CHANNEL="stable"
fi
echo "--- Promoting packages from '${channel}' to '${PROMOTE_CHANNEL}'"

echo "--- :habicat: Retrieving package list from Builder"

channel_pkgs_json=$(curl "https://bldr.habitat.sh/v1/depot/channels/core/${channel}/pkgs")

# TODO (CM): consider ordering these somehow (e.g., save the
# supervisor for absolute last. If it goes out first, Builder itself
# can have a hiccup while _it_ is updating, taking the API out so
# subsequent promotions don't go through.
#
# That's also a good argument for making this step retriable.

non_supervisor_packages=($(echo "${channel_pkgs_json}" | \
                           jq -r \
                             '.data |
                             map(select(.name != "hab-sup")) |
                             map(.origin + "/" + .name + "/" + .version + "/" + .release)
                             | .[]'))

supervisor_packages=($(echo "${channel_pkgs_json}" | \
                       jq -r \
                         '.data |
                         map(select(.name == "hab-sup")) |
                         map(.origin + "/" + .name + "/" + .version + "/" + .release)
                         | .[]'))

for pkg in "${non_supervisor_packages[@]}"; do
    echo "--- :habicat: Promoting '$pkg' to '$PROMOTE_CHANNEL'"
    hab pkg promote --auth="${HAB_TEAM_AUTH_TOKEN}" "${pkg}" "${PROMOTE_CHANNEL}"
done

echo "--- :warning: PROMOTING SUPERVISORS TO '$PROMOTE_CHANNEL' :warning:"
for pkg in "${supervisor_packages[@]}"; do
    echo "--- :habicat: Promoting $pkg to $PROMOTE_CHANNEL"
    hab pkg promote --auth="${HAB_TEAM_AUTH_TOKEN}" "${pkg}" "${PROMOTE_CHANNEL}"
done

buildkite-agent annotate --style="success" --context="release-manifest"

echo "--- :thumbsup: Done!"
