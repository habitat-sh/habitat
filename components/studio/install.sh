#!/bin/sh

# Fail if there are any unset variables and whenever a command returns a
# non-zero exit code.
set -eu
# If the variable `$DEBUG` is set, then print the shell commands as we execute.
if [ -n "${DEBUG:-}" ]; then set -x; fi

# Download URL for the `core/hab-static` Habitat artifact
hart_url="${BPM_HART_URL:-http://s3-us-west-2.amazonaws.com/habitat-sh/core-hab-static-0.5.0-20160520154538-x86_64-linux.hart}"
# Shasum for the Habitat artifact, used to verify the download
hart_sha="${BPM_HART_SHASUM:-46a63f405af2e138e40e3271dfc2ca4b0667cf3af573a0675596910ac7105006}"
# Download location of the Habitat artifact
hart_file="${TMPDIR:-/tmp}/$(basename $hart_url)"

# Add a trap to clean up any interrupted file downloads
trap 'rm -f $hart_file; exit $?' INT TERM EXIT

# Download and verify Habitat artifact by comparing checksums
rm -f $hart_file
wget $hart_url -O $hart_file
checksum=$(sha256sum $hart_file | cut -d ' ' -f 1)
if [ "$hart_sha" != "$checksum" ]; then
  >&2 echo ">>> Checksum invalid for $hart_file"
  >&2 echo ">>> (Expected: $hart_sha  Computed: $checksum)"
  exit 1
fi

# Extract hart into destination, ignoring the signed header info
tail -n +6 $hart_file | xzcat | tar xf - -C /
# Add symlink for convenience under `/bin`
/$(tail -n +6 $hart_file | xzcat | tar t | head -n 1)bin/hab \
  pkg binlink core/hab-static hab

# Clear the file download and extraction clean trap
trap - INT TERM EXIT
rm -f $hart_file

# Install latest hab release and add update symlink
hab install core/hab-static && hab pkg binlink core/hab-static hab
# Install Habitat Studio and add a `hab-studio` symlink under `/bin`
hab install core/hab-studio && hab pkg binlink core/hab-studio hab-studio
