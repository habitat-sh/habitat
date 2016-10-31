#!/bin/bash
set -eux

VERSION="0.7.0"
RELEASE="20160614230104"
ARCH="x86_64"
KERNEL="linux"
PLATFORM="${ARCH}-${KERNEL}"

sudo adduser --group hab || echo "Group 'hab' already exists"
sudo useradd -g hab hab || echo "User 'hab' already exists"

sudo apt-get install curl -y

# Fail if there are any unset variables and whenever a command returns a
# non-zero exit code.
set -eu
# If the variable `$DEBUG` is set, then print the shell commands as we execute.
if [ -n "${DEBUG:-}" ]; then set -x; fi

# Download URL for the `core/hab` Habitat artifact
hab_url="https://bintray.com/habitat/stable/download_file?file_path=${KERNEL}%2F${ARCH}%2Fhab-${VERSION}-${RELEASE}-${PLATFORM}.tar.gz"
# Down URL for the shasum digest
sha_url="${hab_url}.sha256sum"
# Download location for the temporary files
workdir="${TMPDIR:-/tmp}/hab"

# Add a trap to clean up any interrupted file downloads
trap 'rm -rf $workdir; exit $?' INT TERM EXIT

# Download the Habitat slim archive and its shasum digest
rm -rf "$workdir"
mkdir -p "$workdir"
cd "$workdir"
curl -L $hab_url -o $workdir/hab.tar.gz
curl -L $sha_url -o $workdir/hab.tar.gz.sha256sum

# Set the target file name for the slim archive
archive="$workdir/$(cat hab.tar.gz.sha256sum | cut -d ' ' -f 3)"
mv -v "$workdir/hab.tar.gz" "$archive"
# Set the target file name for the shasum digest
sha_file="${archive}.sha256sum"
mv -v "$workdir/hab.tar.gz.sha256sum" "${archive}.sha256sum"

# Verify the provided shasum digest matches the downloaded archive
sha256sum -c "$sha_file"

# Extract the archive into a temporary directory
zcat "$archive" | tar x -C "$workdir"
# Directory containing the binary
archive_dir="$(echo $archive | sed 's/.tar.gz$//')"

sudo "$archive_dir/hab" install core/hab > /dev/null 2>&1
sudo "$archive_dir/hab" pkg binlink core/hab hab
sudo hab install core/hab-sup > /dev/null 2>&1
sudo hab install core/hab-director > /dev/null 2>&1
# JW TODO: give director the same treatment as sup in hab
sudo hab pkg binlink core/hab-director hab-director
