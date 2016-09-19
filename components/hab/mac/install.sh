#!/bin/sh

# Fails on unset variables & whenever a command returns a non-zero exit code.
set -eu
# If the variable `$DEBUG` is set, then print the shell commands as we execute.
if [ -n "${DEBUG:-}" ]; then set -x; fi

# Download URL for a `core/hab` Habitat slim archive
_url='https://api.bintray.com/content/habitat/stable/darwin/x86_64/hab-$latest-x86_64-darwin.zip'
_q="?bt_package=hab-x86_64-darwin"
hab_url="${_url}$_q"
# Download URL for the shasum digest
sha_url="${_url}.sha256sum$_q"
# Download location for the temporary files
workdir="${TMPDIR:-/tmp}/hab"

# Add a trap to clean up any interrupted file downloads
trap 'rm -rf $workdir; exit $?' INT TERM EXIT

# Download the Habitat slim archive and its shasum digest
rm -rf "$workdir"
mkdir -p "$workdir"
cd "$workdir"
curl -sL $hab_url > $workdir/hab-latest.zip
curl -sL $sha_url > $workdir/hab-latest.zip.sha256sum

# Set the target file name for the slim archive
archive="$workdir/$(cat hab-latest.zip.sha256sum | cut -d ' ' -f 3)"
mv -v "$workdir/hab-latest.zip" "$archive"
# Set the target file name for the shasum digest
sha_file="${archive}.sha256sum"
mv -v "$workdir/hab-latest.zip.sha256sum" "${archive}.sha256sum"

# If gnupg is available, verify that the shasum digest is properly signed
if command -v gpg >/dev/null; then
  sha_sig_url="${_url}.sha256sum.asc$_q"
  sha_sig_file="${archive}.sha256sum.asc"
  key_url="https://bintray.com/user/downloadSubjectPublicKey?username=habitat"
  key_file="$workdir/habitat.asc"
  curl -sL "$sha_sig_url" > "$sha_sig_file"
  curl -sL "$key_url" > "$key_file"
  gpg --no-permission-warning --dearmor "$key_file"
  gpg --no-permission-warning --keyring "${key_file}.gpg" --verify "$sha_sig_file"
fi

# Verify the provided shasum digest matches the downloaded archive
shasum -a 256 -c "$sha_file"

# Extract the archive into a temporary directory
unzip "$archive" -d "$workdir"
# Directory containing the binary
archive_dir="$(echo $archive | sed 's/.zip$//')"
# Install latest hab release
install -v "$archive_dir/hab" /bin/hab
