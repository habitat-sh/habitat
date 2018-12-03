#!/bin/bash

set -euo pipefail

source .buildkite/scripts/shared.sh

# Promote the contents of the release channel into the channel
# specified by the argument.
#
# This is to support the usecase of promoting packages from the
# release channel to `builder-live`, and then from `builder-live` to
# `stable`.
#
# In an ideal world, both channels would be arguments, but since we
# don't yet have an API that allows us to ask for only the latest
# packages for each target platform in a given channel, that second
# promotion from `builder-live` to `stable` would get larger and
# larger as time goes on.
#
# Instead, we'll always promote from the release channel, since the
# release pipeline completely controls its lifecycle from creation to
# destruction. We know that all packages in there should be promoted.
#
# In the future, though, we'll want to just promote from
# `builder-live` into `stable`. (Ultimately, of course, we'll want to
# dispense with this `builder-live` stuff altogether because we'll
# have enough additional testing and usage that we *know* it'll be
# perfectly safe to promote a set of release candidates directly to
# `stable`... it'll happen!)
#
# Assumes that the only contents of the channel are going to be
# Habitat Supervisor release artifacts
#
# TODO: It'd be nice to have this be an API function.
to_channel=${1}
from_channel=$(get_release_channel)

echo "--- :thinking_face: Determining which channel to promote to"
if is_fake_release; then
    echo "This isn't a \"real\" release!"
    to_channel="fake-${to_channel}-$(get_fake_release)"
fi

echo "--- Promoting packages from '${from_channel}' to '${to_channel}'"

echo "--- :habicat: Retrieving package list from Builder"

channel_pkgs_json=$(curl "https://bldr.habitat.sh/v1/depot/channels/core/${from_channel}/pkgs")

# The API currently is paginated, 50 packages per page. We should
# never have more than one page of packages (it should be much
# smaller, actually). If we ever have more than one page, call that
# out here.
#
# As we add more platforms and build our packages on new platforms,
# this may change (we may legitimately have more than 50 packages). At
# that point, we can take other approaches.
#
# Numbers are 0-indexed, so the first page would have range_start=0,
# range_end=49, so we have to add 1 to deal with the 0-indexing.
if ! echo "${channel_pkgs_json}" | jq -e '.range_end - .range_start + 1 == .total_count'; then
    echo "There is more than a single API page's worth of packages in the '${from_channel}' channel; something is wrong!"
    exit 1
fi

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
    hab pkg promote --auth="${HAB_AUTH_TOKEN}" "${pkg}" "${to_channel}"
done

echo "--- :warning: PROMOTING SUPERVISORS TO '$to_channel' :warning:"
for pkg in "${supervisor_packages[@]}"; do
    echo "--- :habicat: Promoting $pkg to $to_channel"
    hab pkg promote --auth="${HAB_AUTH_TOKEN}" "${pkg}" "${to_channel}"
done

buildkite-agent annotate --style="success" --context="release-manifest"

echo "--- :thumbsup: Done!"
