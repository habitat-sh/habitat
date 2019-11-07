#!/usr/bin/env bash
# This is a lightweight test to verify a studio can be created before merging a PR.
# This (hopefully) prevents us spending time building the first half of a release 
# only to hit a broken studio. 
# 
# Failure case: because this creates a studio from source, we don't exercise changes
# in our plan.sh, and could still end up with a bad studio build.


set -euo pipefail

export HAB_LICENSE="accept-no-persist"

sudo hab pkg install core/busybox-static core/hab core/hab-backline

# Current studio has the expectation that busybox and hab live in the libexec directroy
# These two lines should be removed at a later date to validate this is no longer a requirement
# While not explicily mentioned, resolving habitat-sh/habitat#6509 will likely remove this 
# explicit requirement.
cp "$(hab pkg path core/busybox-static)"/bin/busybox libexec/busybox
cp "$(hab pkg path core/hab)"/bin/hab libexec/hab

HAB_STUDIO_BACKLINE_PKG="$(< "$(hab pkg path core/hab-backline)"/IDENT)"

export HAB_STUDIO_BACKLINE_PKG

sudo --preserve-env bin/hab-studio.sh new

rm libexec/hab
rm libexec/busybox
