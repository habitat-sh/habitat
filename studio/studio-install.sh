#!/bin/sh

# Fail if there are any unset variables and whenever a command returns a
# non-zero exit code.
set -eu
# If the variable `$DEBUG` is set, then print the shell commands as we execute.
if [ -n "${DEBUG:-}" ]; then set -x; fi

# Download URL for the `chef/bpm` slim package
bpm_tar_url="${BPM_TAR_URL:-http://s3-us-west-2.amazonaws.com/fnichol-lfs-tools/chef-bpm-0.1.0-20160307233938.tar.xz}"
# Shasum for the slim package, used to verify the download
bpm_tar_sha="${BPM_TAR_SHASUM:-43e39868ab5da229457f6128bed2da58533326cb331589fec8ba6a0dd2f7f61d}"

# Download location of the slim package
bpm_tar_file="${TMPDIR:-/tmp}/$(basename $bpm_tar_url)"
# The destination path for the extracted slim package
dir="/opt/bldr/support/$(echo $(basename $bpm_tar_file)| sed 's,\.tar\.xz$,,')"

# Add a trap to clean up any interrupted file downloads
trap 'rm -f $bpm_tar_file; exit $?' INT TERM EXIT

# Download and verify tarball
rm -f $bpm_tar_file
echo "Downloading $(basename $bpm_tar_file)"
wget $bpm_tar_url -O $bpm_tar_file
checksum=$(sha256sum $bpm_tar_file | cut -d ' ' -f 1)
if [ "$bpm_tar_sha" != "$checksum" ]; then
  >&2 echo ">>> Checksum invalid for $bpm_tar_file"
  >&2 echo ">>> (Expected: $bpm_tar_sha  Computed: $checksum)"
  exit 1
fi

# Extract tarball into destination
mkdir -p $(dirname $dir)
rm -rf $dir
xzcat $bpm_tar_file | tar xf - -C $(dirname $dir)

# Add symlink for convenience
mkdir -p /usr/bin
ln -snf $dir/bin/bpm /usr/bin/bpm

# Clear the file download and extraction clean trap
trap - INT TERM EXIT

# Install Bldr Studio and add a `studio` symlink to `/usr/bin/studio`
if ! command -v studio > /dev/null; then
  bpm install chef/bldr-studio
  bpm binlink chef/bldr-studio studio
fi
echo "$(/usr/bin/studio version) is installed."
