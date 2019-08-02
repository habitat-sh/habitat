#!/bin/bash

set -euo pipefail

print_help() {
    program=$(basename "$0")
    echo "$program

Test hab pkg tar export

USAGE:
        $program <PKG_IDENT>

ARGS:
    <PKG_IDENT>  The origin and name of the package to schedule a job for (eg: core/redis) or
                 A fully qualified package identifier (ex: core/busybox-static/1.42.2/20170513215502)
"
}

if [ -n "${DEBUG:-}" ]; then
    set -x
    export DEBUG
fi

if [[ $# -eq 0 ]] ; then
    print_help
    echo
    echo "<PKG_IDENT> must be specified"
    exit 1
else
    pkg_ident="$1"
fi


channel="${2}"

# Remove tarball if already present
rm -f ./*.tar.gz

hab pkg export tar "$pkg_ident" --base-pkgs-channel="${channel}"

# Check if tarball is present

if [ "$(find . -maxdepth 1 -type f -name "*.tar.gz")" ] ; then
    echo "--- Package was successfully exported to a tarball"
else
    echo "--- Package was NOT successfully exported"
    exit 1
fi

echo "--- Woo!"
