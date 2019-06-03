#!/usr/bin/env bash

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
export STUDIO_COMMAND="sudo --preserve-env $(realpath bin/hab-studio.sh)"

./test/shared/test-all.sh


rm libexec/hab
rm libexec/busybox
