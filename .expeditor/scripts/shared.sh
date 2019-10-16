#!/bin/bash

# This is the bucket that all manifests, hart files, and other
# assorted artifacts will be uploaded to.
#
# Despite the name (an accident of history), we store more than just
# Automate-related material there.
readonly s3_bucket_name="chef-automate-artifacts"

# Always install the latest hab binary appropriate for your linux platform
#
# This will install `hab` to the place appropriate for the target
curlbash_hab() {
    local pkg_target="${1:-$BUILD_PKG_TARGET}"
    echo "--- :habicat: Bootstrap installation of the current stable hab binary for $pkg_target using curl|bash"
    # TODO:
    # really weird corner case on linux2 because the 0.82.0 versions of both
    # are the same. let's just delete it
    #
    # This command is potentially executed on different queues, under different users
    # with varying levels of permissions.  Attempt to sudo-remove it first, for the linux-privileged
    # use case, and if that fails, try to remove it directly for the docker use case.
    if [ -d /hab/pkgs/core/hab/0.82.0 ]; then
      sudo rm -rf /hab/pkgs/core/hab/0.82.0 || \
           rm -rf /hab/pkgs/core/hab/0.82.0
    fi
    curl https://raw.githubusercontent.com/habitat-sh/habitat/master/components/hab/install.sh | sudo bash -s -- -t "$pkg_target"
    case "${pkg_target}" in
        x86_64-linux | x86_64-linux-kernel2)
            hab_binary="/bin/hab"
            ;;
        x86_64-darwin)
            hab_binary="/usr/local/bin/hab"
            ;;
        *)
            echo "--- :no_entry_sign: Unknown PackageTarget: ${pkg_target}"
            exit 1
            ;;
    esac
    echo "--- :habicat: Hab binary set to $hab_binary"
}

get_toolchain() {
    dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"
    cat "$dir/../../rust-toolchain"
}

s3_upload_file() {
    local file_name="${1:?}"
    local s3_url="${2:?}"
    aws --profile chef-cd s3 cp "$file_name" "$s3_url" --acl public-read
}

# Sets the given manifest `file` to be the current one for the given
# environment.
upload_manifest_for_environment() {
    local file="${1}"
    local environment_name="${2}"
    s3_upload_file "$file" "s3://${s3_bucket_name}/${environment_name}/latest/habitat/manifest.json"
}

# Retrieves the current package manifest for the given environment.
#
# Returns the JSON on standard output, suitable for piping
# into `jq`.
#
# e.g. manifest_for_environment acceptance | jq
manifest_for_environment() {
    local environment_name="${1:?}"
    curl --silent "http://${s3_bucket_name}.s3.amazonaws.com/${environment_name}/latest/habitat/manifest.json"
}
