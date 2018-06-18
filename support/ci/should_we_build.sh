#!/bin/bash
#
# Given the name of a Habitat package in this repository, consult Git
# to determine if any files that are used by that package have
# changed. If so, exit with 0 (meaning that we should build), or with
# 1 (meaning we should not build).

set -u

# e.g., "builder-api", "hab-pkg-export-docker", etc.
package_name="${1}"

echo "======================================================="
echo "Testing whether or not we need to build ${package_name}"
echo "======================================================="

cat .bldr.toml | \
    rq --input-toml | \
    jq --exit-status '.["'${package_name}'"]' &>/dev/null

if [ $? != 0 ]; then
    echo "No project named ${package_name} in .bldr.toml!"
    exit 1
fi

CHANGED_FILES="$(support/ci/what_changed.sh)"

# Consult `.bldr.toml` to figure out which files we care about for the
# given package.

# Requires `rq` and `jq` to be installed for TOML parsing

# Given input TOML like this:
#
#    [hab-sup]
#    plan_path = "components/sup"
#    paths = [
#      "components/sup/*",
#      "components/eventsrv-client/*",
#      "components/launcher-client/*",
#      "components/butterfly/*",
#      "components/core/*",
#      "components/builder-depot-client/*",
#    ]
#
# we extract the plan path and turn it into a regex that matches that
# root directory (i.e., given `components/sup`, create
# `components/sup/*`), then merge that in with all the other paths we
# care about. Then, we ensure they're all unique, and then create one
# big regex by joining it all together with `|`
#
# Thus we are left with:
#
#    components/builder-depot-client/*|components/butterfly/*|components/core/*|components/eventsrv-client/*|components/launcher-client/*|components/sup/*
#
# (Note: I wanted to do this with just `rq`, but I couldn't find a way
# to join the *modified* `plan_path` together with the contents of
# `path. Thus, all it's doing here is converting TOML to JSON so we
# can use `jq`'s more flexible language to process the data. `jq` can
# also output raw strings, so we don't need a `tr -d '"'` tacked on to
# the end.)

search_expression=$(cat .bldr.toml | \
    rq --input-toml | \
    # All entries should have a `plan_path` key; not all will have `paths`
    jq -r '[.["'${package_name}'"]["plan_path"] + "/*", .["'${package_name}'"]["paths"] // empty] |
       flatten |
       sort |
       unique |
       join("|")')

echo
echo "The following files have changes since the last merge commit:"
echo
echo "$CHANGED_FILES" | sed 's,^,    ,g'
echo

# If any of these "master files" change, it will affect the build of EVERY package.
# (Note: this is a regular expression, and should be escaped as such)
master_files="\.travis\.yml|\.bldr\.toml|support/ci/*|Cargo\.toml|Cargo\.lock|VERSION"

echo "Performing a preliminary check for changes in the following \"master\" files:"
echo
# All this sed is to get the individual regex options to print nicely
echo "    $master_files" | sed "s,|,\n    ,g" | sed 's,\\,,g'
echo

if echo "${CHANGED_FILES}" | grep --quiet --extended-regexp "^(${master_files})" ; then
    echo "A master file has changed; we should build ${package_name} regardless"
    exit 0
fi

echo "No master files changed; checking for changes in the following paths from .bldr.toml for ${package_name}:"
echo
# This sed is to get the individual paths to print nicely; nothing
# is escaped, though, so it's simpler than last time
echo "    $search_expression" | sed "s,|,\n    ,g"
echo

if echo "$CHANGED_FILES" | grep --quiet --extended-regexp "^(${search_expression})" ; then
    echo "Files for ${package_name} have changed; we should build the package"
    exit 0
fi

echo "No files for ${package_name} have changed; we should *not* build the package"
exit 1
