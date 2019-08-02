#!/bin/bash

set -euo pipefail

# Test the Habitat tar exporter.
#
# Pass as arguments the name of the package to export, and the channel
# from which to pull the base packages (supervisor, launcher, etc.)
#
# Example:
#
#     test_tar_export.sh core/gzip stable
#

pkg_ident="${1}"
channel="${2}"

# Remove tarball if already present
rm -f ./*.tar.gz

hab pkg export tar "${pkg_ident}" --base-pkgs-channel="${channel}"

tarball=$(find . -maxdepth 1 -type f -name "*.tar.gz")

# Check if tarball is present
if [ -f "${tarball}" ] ; then
    echo "--- Package was successfully exported to a tarball"
else
    echo "--- Package was NOT successfully exported"
    exit 1
fi

echo "--- Woo!"
