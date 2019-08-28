#!/bin/bash

# Unpack the hart file from our channel, repack it, and upload it to
# package-router

set -euo pipefail

source .expeditor/scripts/release_habitat/shared.sh

export HAB_AUTH_TOKEN="${ACCEPTANCE_HAB_AUTH_TOKEN}"
export HAB_BLDR_URL="${ACCEPTANCE_HAB_BLDR_URL}"

channel=$(get_release_channel)

echo "--- Channel: $channel - bldr url: $HAB_BLDR_URL"

release_version=$(get_latest_pkg_release_in_channel "hab")

declare -g hab_binary
install_release_channel_hab_binary "x86_64-linux"

${hab_binary} pkg install core/gzip
${hab_binary} pkg install core/tar
${hab_binary} pkg install core/wget
${hab_binary} pkg install core/zip

# Import packages@chef.io GPG signing key
aws s3 cp s3://chef-cd-citadel/packages_at_chef.io.pgp packages_at_chef.io.pgp --profile=chef-cd
gpg --import packages_at_chef.io.pgp

#TODO make this nicer
tmp_root="$(mktemp -d -t "grant-XXXX")"
extract_dir="$tmp_root/extract"
mkdir -p "$extract_dir"

echo "--- Downloading $release_version for $BUILD_PKG_TARGET"
if [[ $BUILD_PKG_TARGET == *"darwin" ]]; then
  artifact_name=$(buildkite-agent meta-data get MACOS_ARTIFACT)
  buildkite-agent artifact download "$artifact_name" "$tmp_root/" --step "[:macos: build hab]"
  mv "$tmp_root/$artifact_name" "$tmp_root/hab-$channel.hart"
else
  hab pkg exec core/wget wget "${HAB_BLDR_URL}/v1/depot/pkgs/core/hab/$release_version/download?target=$BUILD_PKG_TARGET" -O "$tmp_root/hab-$channel.hart"
fi

target_hart="$tmp_root/hab-$channel.hart"
tail -n+6 "${target_hart}" | \
    tar --directory "${extract_dir}" \
        --extract \
        --xz \
        --strip-components=6

extracted_hab_binary="$(find "$extract_dir" \( -name hab -or -name hab.exe \) -type f)"
pkg_target="$(tr --delete '\r' < "${extract_dir}"/TARGET)"
pkg_arch="$(echo "$pkg_target" | cut -d '-' -f 1)"
pkg_kernel="$(echo "$pkg_target" | cut -d '-' -f 2)"
pkg_ident="$(tr --delete '\r' < "$extract_dir"/IDENT)"
pkg_origin="$(echo "$pkg_ident" | cut -d '/' -f 1)"
pkg_name="$(echo "$pkg_ident" | cut -d '/' -f 2)"
pkg_version="$(echo "$pkg_ident" | cut -d '/' -f 3)"
pkg_release="$(echo "$pkg_ident" | cut -d '/' -f 4)"
archive_name="hab-$(echo "$pkg_ident" | cut -d '/' -f 3-4 | tr '/' '-')-$pkg_target"

build_dir="$tmp_root/build"
pkg_dir="$build_dir/${archive_name}"

echo "Copying $extracted_hab_binary to $(basename "$pkg_dir")"
mkdir -p "$pkg_dir"
mkdir -p "$tmp_root/results"

if [[ $pkg_target == *"windows" ]]; then
for file in "$(dirname "$extracted_hab_binary")"/*; do 
  cp -p "$file" "$pkg_dir/"
done
else
  cp -p "$extracted_hab_binary" "$pkg_dir/$(basename "$extracted_hab_binary")"
fi

echo "Compressing \`hab' binary"
pushd "$build_dir" >/dev/null
case "$pkg_target" in
*-linux | *-linux-kernel2)
    pkg_artifact="$tmp_root/results/${archive_name}.tar.gz"
    tarball="$build_dir/$(basename "${pkg_artifact%.gz}")"
    hab pkg exec core/tar tar cf "$tarball" "$(basename "$pkg_dir")"
    rm -fv "$pkg_artifact"
    hab pkg exec core/gzip gzip -9 -c "$tarball" > "$pkg_artifact"
    ;;
*-darwin | *-windows)
    pkg_artifact="$tmp_root/results/${archive_name}.zip"
    rm -fv "$pkg_artifact"
    hab pkg exec core/zip zip -9 -r "$pkg_artifact" "$(basename "$pkg_dir")"
    ;;
*)
    exit_with "$target_hart has unknown TARGET=$pkg_target" 3
    ;;
esac

# Generate our shasum
popd >/dev/null
pushd "$(dirname "$pkg_artifact")" >/dev/null
sha256sum "$(basename "$pkg_artifact")" > "${pkg_artifact}.sha256sum"

# Sign our artifact
popd >/dev/null
pushd "$(dirname "$pkg_artifact")" >/dev/null
gpg --armor --digest-algo sha256 --default-key 2940ABA983EF826A --output "$(basename "$pkg_artifact").asc" --detach-sign "$(basename "$pkg_artifact")"
popd

# Name of the file to upload
upload_artifact="$(basename "$pkg_artifact")"
# Now strip the `release` so we have a 'latest'
latest_artifact="$(basename "$pkg_artifact" | sed -E 's/[0-9]{14}/latest/')"

echo "--- Uploading $upload_artifact and associated artifacts to S3"
pushd "$(dirname "$pkg_artifact")" >/dev/null
# FYI - the bucket name is not just for automate artifacts, and this will be fixed up later
# Upload unstable/latest
aws --profile chef-cd s3 cp "$upload_artifact" "s3://chef-automate-artifacts/unstable/latest/habitat/$latest_artifact" --acl public-read
aws --profile chef-cd s3 cp "$upload_artifact.asc" "s3://chef-automate-artifacts/unstable/latest/habitat/$latest_artifact.asc" --acl public-read
aws --profile chef-cd s3 cp "$upload_artifact.sha256sum" "s3://chef-automate-artifacts/unstable/latest/habitat/$latest_artifact.sha256sum" --acl public-read
# Upload versioned
aws --profile chef-cd s3 cp "$upload_artifact" "s3://chef-automate-artifacts/files/habitat/$release_version/$upload_artifact" --acl public-read
aws --profile chef-cd s3 cp "$upload_artifact" "s3://chef-automate-artifacts/files/habitat/$release_version/$upload_artifact.asc" --acl public-read
aws --profile chef-cd s3 cp "$upload_artifact" "s3://chef-automate-artifacts/files/habitat/$release_version/$upload_artifact.sha256sum" --acl public-read
popd