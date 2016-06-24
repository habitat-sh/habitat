#!/bin/bash
set -eux

VERSION="0.7.0"
RELEASE="20160614230104"
PLATFORM="x86_64-linux"

sudo adduser --group hab || echo "Group 'hab' already exists"
sudo useradd -g hab hab || echo "User 'hab' already exists"

sudo apt-get install curl -y

# Fail if there are any unset variables and whenever a command returns a
# non-zero exit code.
set -eu
# If the variable `$DEBUG` is set, then print the shell commands as we execute.
if [ -n "${DEBUG:-}" ]; then set -x; fi

# Download URL for the `core/hab` Habitat artifact
hart_url="http://s3-us-west-2.amazonaws.com/habitat-sh/core-hab-${VERSION}-${RELEASE}-${PLATFORM}.hart"
# Shasum for the Habitat artifact, used to verify the download
hart_sha="8d7e96ff0ae1b5783f4c88370b9e94503efec3c69bc9cd58ccec574a610d52d4"
# Download location of the Habitat artifact
hart_file="/tmp/$(basename $hart_url)"

# Add a trap to clean up any interrupted file downloads
trap 'rm -f $hart_file; exit $?' INT TERM EXIT

# Download and verify Habitat artifact by comparing checksums
rm -f $hart_file
curl -L $hart_url -o $hart_file
checksum=$(sha256sum $hart_file | cut -d ' ' -f 1)
if [ "$hart_sha" != "$checksum" ]; then
  >&2 echo ">>> Checksum invalid for $hart_file"
  >&2 echo ">>> (Expected: $hart_sha  Computed: $checksum)"
  exit 1
fi

# Extract hart into destination, ignoring the signed header info
tail -n +6 $hart_file | xzcat | sudo tar xf - -C /
# Add symlink for convenience under `/bin`
sudo /$(tail -n +6 $hart_file | xzcat | sudo tar t | head -n 1)bin/hab pkg binlink core/hab hab

# Clear the file download and extraction clean trap
trap - INT TERM EXIT
rm -f $hart_file

sudo hab install core/hab > /dev/null 2>&1
sudo hab install core/hab-sup > /dev/null 2>&1
sudo hab install core/hab-director > /dev/null 2>&1
sudo hab pkg binlink core/hab hab
# JW TODO: give director the same treatment as sup in hab
sudo hab pkg binlink core/hab-director hab-director
