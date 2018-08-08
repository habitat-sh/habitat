#!/bin/bash

set -euo pipefail

source .buildkite/scripts/shared.sh

# Given a FROM channel and a TO channel, take the contents of FROM and
# promote to TO
#
# "release-channel" -> builder-live
# builder-live -> stable
#
# Assumes that the only contents of the channel are going to be
# Habitat Supervisor release artifacts
#
# TODO: It'd be nice to have this be an API function.

from_channel=${1}
to_channel=${2}

echo "--- :thinking_face: Determining which channel to promote to"
if is_fake_release; then
    echo "This isn't a \"real\" release!"
    to_channel="fake-${to_channel}-$(get_fake_release)"

    # Yeah, this is kinda gross, but it'll allow us to test the whole
    # thing (otherwise we wouldn't get the behavior we're after when
    # doing the "second stage" promotion, since the name wouldn't match).
    #
    # Other suggestions welcome!
    if [[ "${from_channel}" == "builder-live" ]]; then
        from_channel="fake-builder-live-$(get_fake_release)"
    fi
fi

echo "--- Promoting packages from '${from_channel}' to '${to_channel}'"

echo "--- :habicat: Retrieving package list from Builder"

channel_pkgs_json=$(curl "https://bldr.habitat.sh/v1/depot/channels/core/${from_channel}/pkgs")

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
    echo "--- :habicat: Promoting '$pkg' to '$to_channel'"
    hab pkg promote --auth="${HAB_TEAM_AUTH_TOKEN}" "${pkg}" "${to_channel}"
done

echo "--- :warning: PROMOTING SUPERVISORS TO '$to_channel' :warning:"
for pkg in "${supervisor_packages[@]}"; do
    echo "--- :habicat: Promoting $pkg to $to_channel"
    hab pkg promote --auth="${HAB_TEAM_AUTH_TOKEN}" "${pkg}" "${to_channel}"
done

buildkite-agent annotate --style="success" --context="release-manifest"

echo "--- :thumbsup: Done!"
