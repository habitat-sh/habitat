#!/bin/bash

# Unpack the hart file from our channel, repack it, and upload it to
# package-router

set -euo pipefail

source .expeditor/scripts/release_habitat/shared.sh

export HAB_AUTH_TOKEN="${ACCEPTANCE_HAB_AUTH_TOKEN}"
export HAB_BLDR_URL="${ACCEPTANCE_HAB_BLDR_URL}"

channel=$(get_release_channel)

echo "--- Channel: $channel - bldr url: $HAB_BLDR_URL"

release_version=$(get_latest_pkg_release_version_in_release_channel "hab")

declare -g hab_binary
install_release_channel_hab_binary "x86_64-linux"

${hab_binary} pkg install core/gzip \
  core/tar \
  core/wget \
  core/zip

# Import packages@chef.io GPG signing key
aws s3 cp s3://chef-cd-citadel/packages_at_chef.io.pgp \
  packages_at_chef.io.pgp \
  --profile=chef-cd 
gpg --import packages_at_chef.io.pgp

tmp_root="$(mktemp -d -t "grant-XXXX")"
extract_dir="$tmp_root/extract"
mkdir -p "$extract_dir"

echo "--- Downloading $release_version for $BUILD_PKG_TARGET"
if [[ $BUILD_PKG_TARGET == *"darwin" ]]; then
  artifact_name=$(buildkite-agent meta-data get MACOS_ARTIFACT)
  buildkite-agent artifact download "$artifact_name" "$tmp_root/" --step "[:macos: build hab]"
  mv "$tmp_root/$artifact_name" "$tmp_root/hab-$channel.hart"
else
  # once https://github.com/habitat-sh/habitat/issues/6878 is released,
  # we can use hab pkg download rather than this wget
  hab pkg exec core/wget wget \
    "${HAB_BLDR_URL}/v1/depot/pkgs/core/hab/$release_version/download?target=$BUILD_PKG_TARGET" \
    -O "$tmp_root/hab-$channel.hart"
fi

target_hart="$tmp_root/hab-$channel.hart"
if [[ $(head -n 1 "${target_hart}") != HART-1 ]]; then
  echo "Hart file does not match expected format, exiting."
  exit 1
fi
tail -n+6 "${target_hart}" | \
    tar --directory "${extract_dir}" \
        --extract \
        --xz \
        --strip-components=6

extracted_hab_binary="$(find "$extract_dir" \( -name hab -or -name hab.exe \) -type f)"
pkg_ident="$(tr --delete '\r' < "$extract_dir"/IDENT)"
pkg_version="$(echo "$pkg_ident" | cut -d '/' -f 3)"
pkg_release="$(echo "$pkg_ident" | cut -d '/' -f 4)"
archive_name="hab-${pkg_version}-${pkg_release}-$BUILD_PKG_TARGET"

build_dir="$tmp_root/build"
pkg_dir="$build_dir/${archive_name}"

echo "Copying $extracted_hab_binary to $(basename "$pkg_dir")"
mkdir -p "$pkg_dir"
mkdir -p "$tmp_root/results"

if [[ $BUILD_PKG_TARGET == *"windows" ]]; then
  # windows has more than just the hab binary, so we need to copy all the files
  # in the directory
  for file in "$(dirname "$extracted_hab_binary")"/*; do 
    cp -p "$file" "$pkg_dir/"
  done
else
  cp -p "$extracted_hab_binary" "$pkg_dir/$(basename "$extracted_hab_binary")"
fi

echo "Compressing 'hab' binary"
(cd "$build_dir"
  case "$BUILD_PKG_TARGET" in
  *-linux | *-linux-kernel2)
      pkg_artifact="$tmp_root/results/${archive_name}.tar.gz"
      tarball="$basename --suffix=".gz" "${pkg_artifact}""
      hab pkg exec core/tar tar cf "$tarball" "$(basename "$pkg_dir")"
      hab pkg exec core/gzip gzip -9 -c "$tarball" > "$pkg_artifact"
      ;;
  *-darwin | *-windows)
      pkg_artifact="$tmp_root/results/${archive_name}.zip"
      hab pkg exec core/zip zip -9 -r "$pkg_artifact" "$(basename "$pkg_dir")"
      ;;
  *)
      exit_with "$target_hart has unknown TARGET=$BUILD_PKG_TARGET" 3
      ;;
  esac
)
# Generate our shasum
(cd "$(dirname "$pkg_artifact")"
  sha256sum "$(basename "$pkg_artifact")" > "${pkg_artifact}.sha256sum"
)

# Sign our artifact
(cd "$(dirname "$pkg_artifact")"
gpg --armor \
  --digest-algo sha256 \
  --default-key 2940ABA983EF826A \
  --output "$(basename "$pkg_artifact").asc" \
  --detach-sign "$(basename "$pkg_artifact")"
)

# Name of the file to upload
upload_artifact="$(basename "$pkg_artifact")"
# Now strip the `release` so we have a 'latest'
# If the release ever changes, we'll need to take that into account.
# This should probably be updated to be more robust in the future.
latest_artifact="$(basename "$pkg_artifact" | sed -E 's/[0-9]{14}/latest/')"

echo "--- Uploading $upload_artifact and associated artifacts to S3"
(cd "$(dirname "$pkg_artifact")"
  # FYI - this bucket is not just for automate artifacts, regardless of the name.
  unstable_s3_url="s3://chef-automate-artifacts/unstable/latest/habitat/$latest_artifact"
  s3_upload_file "$upload_artifact" "$unstable_s3_url"
  s3_upload_file "$upload_artifact.asc" "$unstable_s3_url.asc"
  s3_upload_file "$upload_artifact.sha256sum" "$unstable_s3_url.sha256sum"
  # Upload versioned
  versioned_s3_url="s3://chef-automate-artifacts/files/habitat/$release_version/$upload_artifact"
  s3_upload_file "$upload_artifact" "$versioned_s3_url"
  s3_upload_file "$upload_artifact.asc" "$versioned_s3_url.asc"
  s3_upload_file "$upload_artifact.sha256sum" "$versioned_s3_url.sha256sum"
)