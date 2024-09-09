#!/bin/bash

# TODO (CM): for the verify and release pipelines, this should
# probably operate from the install.sh script in the repo itself, right?

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
    # with varying levels of permissions. Attempt to sudo-remove it first, for the linux-privileged
    # use case, and if that fails, try to remove it directly for the docker use case.
    if [ -d /hab/pkgs/core/hab/0.82.0 ]; then
      sudo rm -rf /hab/pkgs/core/hab/0.82.0 || \
           rm -rf /hab/pkgs/core/hab/0.82.0
    fi

    # Use the temporarily uploaded Habitat package from the 'aarch64-linux' channel for aarch64.
    # Once hab is released in the LTS channel, we can use either HAB_BLDR_CHANNEL or HAB_REFRESH_CHANNEL to install hab.
    install_args="-t $pkg_target"
    if [ "$pkg_target" = "aarch64-linux" ]; then
        install_args="$install_args -c aarch64-linux -v 1.6.706"
    fi
    curl https://raw.githubusercontent.com/habitat-sh/habitat/main/components/hab/install.sh | sudo bash -s -- "$install_args"
    case "${pkg_target}" in
        x86_64-linux | aarch64-linux | x86_64-linux-kernel2)
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
    echo "--- :habicat: Hab binary set to $hab_binary (version: $(${hab_binary} --version))"
}

install_rustup() {
  if command -v rustup && command -v cargo &>/dev/null; then
    echo "--- :rust: rustup is currently installed."
  else
    echo "--- :rust: Installing rustup."
    curl https://sh.rustup.rs -sSf | sh -s -- -y --profile=minimal
    # shellcheck disable=SC1090
    source "$HOME"/.cargo/env
  fi
}

get_toolchain() {
    dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"
    tail -n 1 "$dir/../../rust-toolchain" | cut -d'"' -f 2
}

# Chef's GPG key for generating signatures. See `import_gpg_keys`
# and `gpg_sign` below.
readonly chef_gpg_key="2940ABA983EF826A"

# Imports Chef's packages@chef.io GPG keys. This must be called before
# running either `gpg_sign` or `gpg_verify`.
import_gpg_keys() {
    vault kv get \
          -field packages_at_chef_io \
          account/static/packages/signing_certs \
          > packages_at_chef.io.pgp
    gpg --import packages_at_chef.io.pgp
}

# Signs `file` with Chef's GPG key, generating a corresponding
# `*.asc` signature in the same directory as `file`.
gpg_sign() {
    local file="${1}"

    # Adding the `--yes` option here because we've seen cases where an
    # *.asc file is already present somehow, even though that _should_
    # be impossible.
    gpg --armor \
        --digest-algo sha256 \
        --default-key "${chef_gpg_key}" \
        --output "${file}.asc" \
        --yes \
        --detach-sign \
        "${file}"
}

# Verify a file's GPG signature. Assumes a similarly named `asc` file
# is located in the same directory.
gpg_verify(){
    local file="${1}"
    gpg --verify "${file}.asc" "${file}"
}

# Generate a SHA256 checksum of `file`.
checksum_file() {
    local file="${1}"
    sha256sum "${file}" > "${file}.sha256sum"
}

# This is the bucket that all manifests, hart files, and other
# assorted artifacts will be uploaded to.
#
# Despite the name (an accident of history), we store more than just
# Automate-related material there.
readonly s3_bucket_name="chef-automate-artifacts"

# Helper function for running s3 cp with appropriate settings.
s3_cp() {
    local src="${1}"
    local dest="${2}"
    aws --profile chef-cd \
        s3 cp "${src}" "${dest}" \
        --acl public-read
}

s3_file_url_root() {
    local version="${1}"
    echo "s3://${s3_bucket_name}/files/habitat/${version}"
}

s3_channel_url_root() {
    local channel="${1}"
    echo "s3://${s3_bucket_name}/${channel}/latest/habitat"
}

purge_fastly_cache() {
	local service_key="${1}"
	local channel="${2:-}"
	local token

	if [ -n "${FASTLY_API_KEY:-}" ]; then
		token="$FASTLY_API_KEY"
	else
		token=$(fastly_token)
	fi

	if [ -n "$channel" ]; then
		curl -X POST -H "Fastly-Key: ${token}" "https://api.fastly.com/service/$service_key/purge/${channel}/habitat/latest"
	else
		curl -X POST -H "Fastly-Key: ${token}" "https://api.fastly.com/service/$service_key/purge_all"
	fi
}

# Intended for uploading manifests and `hab` packages in non-Habitat
# archives (e.g., tarballs, not harts) to S3.
#
# Artifacts are GPG signed and checksummed, and those files are
# uploaded as well to a versioned directory.
#
# e.g. store_in_s3 0.88.0 hab-x86_64-linux.tar.gz
#      store_in_s3 0.88.0 manifest.json
store_in_s3() {
    # I guess we *could* just call `get_version` right here if we
    # wanted to.
    local version="${1}"
    local artifact="${2}"

    checksum_file "${artifact}"
    gpg_sign "${artifact}"
    gpg_sign "${artifact}.sha256sum"

    local versioned_url
    versioned_url="$(s3_file_url_root "${version}")"

    s3_cp \
        "${artifact}" \
        "${versioned_url}/${artifact}"
    s3_cp \
        "${artifact}.asc" \
        "${versioned_url}/${artifact}.asc"
    s3_cp \
        "${artifact}.sha256sum" \
        "${versioned_url}/${artifact}.sha256sum"
    s3_cp \
        "${artifact}.sha256sum.asc" \
        "${versioned_url}/${artifact}.sha256sum.asc"
}

# Recursively copy all the Habitat artifacts of a given version into
# the specified destination channel as the current "latest"
# artifacts.
#
# This should take care of manifests, hab archives, and all the
# associated signature and checksum files
#
# e.g. promote_version_in_s3 0.88.0 dev
#      promote_version_in_s3 0.88.0 acceptance
promote_version_in_s3() {
    local version="${1}"
    local destination="${2}"
    aws --profile chef-cd \
        s3 cp \
        "$(s3_file_url_root "${version}")" \
        "$(s3_channel_url_root "${destination}")" \
        --recursive \
        --acl public-read
}

# Retrieves the current package manifest for the given environment.
#
# After GPG verifying the file, the file will be present in the
# current directory with the name "manifest.json".
get_manifest_for_environment() {
    local environment_name="${1}"

    local source_root
    source_root="$(s3_channel_url_root "${environment_name}")"

    s3_cp \
        "${source_root}/manifest.json" \
        "manifest.json"

    s3_cp \
        "${source_root}/manifest.json.asc" \
        "manifest.json.asc"

    gpg_verify "manifest.json"
}

# Reads information from a manifest.json file and promotes the Habitat
# packages specified therein to the designated channel in Builder.
promote_packages_to_builder_channel() {
    local manifest="${1}"
    local destination_channel="${2}"

    local manifest_json
    manifest_json=$(cat "${manifest}")
    local version
    version="$(echo "${manifest_json}" | jq -r '.version')"

    mapfile -t targets < <(echo "${manifest_json}" | jq -r ".packages | keys | .[]")

    echo "--- Promoting Habitat packages to the ${destination_channel} channel of ${HAB_BLDR_URL}"
    for target in "${targets[@]}"; do
        mapfile -t idents < <(echo "${manifest_json}" | jq -r ".packages.\"${target}\" | .[]")
        for ident in "${idents[@]}"; do
            echo "--- Promoting ${ident} (${target}) to '${destination_channel}'"
            ${hab_binary} pkg promote \
                          --auth="${HAB_AUTH_TOKEN}" \
                          --url="${HAB_BLDR_URL}" \
                          "${ident}" "${destination_channel}" "${target}"
        done
    done

    echo "--- Sending datadog event for Supervisor promotion"
    local EXIT_CODE=0
    send_channel_promotion_datadog_event "${version}" "${destination_channel}" || EXIT_CODE=$?
    if [ $EXIT_CODE -ne 0 ]; then
      echo "Failed to send datadog event for Supervisor promotion, but continuing anyway."
    fi
}

# Create a datadog event for the promotion of a Supervisor version to a builder channel. 
# 
# e.g. send_channel_promotion_datadog_event 0.88.0 dev
send_channel_promotion_datadog_event() {
    local version="${1}"
    local channel="${2}"
    local DD_CLIENT_API_KEY
    DD_CLIENT_API_KEY=$(vault kv get -field api_key_supervisor account/static/datadog/habitat)

    curl --connect-timeout 5 \
      --max-time 10 \
      --retry 5 \
      --retry-delay 0 \
      --retry-max-time 40 \
      --request POST https://api.datadoghq.com/api/v1/events \
      --header "Expect:" \
      --header "DD-API-KEY: ${DD_CLIENT_API_KEY}" \
      --header 'Content-Type: application/json charset=utf-8' \
      --data-binary @- << EOF
{   
  "aggregation_key":"supervisor_promotion",
  "alert_type":"info",
  "date_happened":$(date "+%s"),
  "priority":"normal",
  "tags":[
    "channel:${channel}"
  ],
  "text":"",
  "title":"Promoted Supervisor '${version}' to '${channel}'"
}
EOF
}

# Retrieves a suitable HAB_AUTH_TOKEN value from Vault.
#
# (Does NOT set that variable, though!)
#
# This is intended for use in scripts that Expeditor runs for us;
# scripts that run in Buildkite can have such credentials injected via
# other means (See https://expeditor.chef.io/docs/reference/secrets-dsl)
#
# Should correspond with `expeditor_hab_bldr_url` below.
hab_auth_token() {
    vault kv get \
          -field=auth_token \
          account/static/habitat/chef-ci
}

# Retrieve a suitable GITHUB_TOKEN value from Vault.
chef_ci_github_token() {
    vault kv get \
          -field=token \
          account/static/github/chef-ci
}

# Our pipelines can define this value as a top-level environment
# variable, but we don't have that ability with scripts executed
# directly by Expeditor. Those scripts use this (similar to
# `hab_auth_token` above).
#
# Not naming it `hab_bldr_url` to avoid Shellcheck complaints about
# possible misspellings, but also to make it very obvious what it is.
#
# Should correspond with `hab_auth_token` above.
readonly expeditor_hab_bldr_url="https://bldr.habitat.sh"

# This allows people to e.g. trigger end-to-end pipeline runs manually
# when iterating on tests, but without having to fear that they'll
# inadvertently promote a set of artifacts accidentally.
#
# Only Chef Expeditor should be triggering "real" runs of pipelines
# that use this script.

maybe_run() {
  if [[ "${BUILDKITE_BUILD_CREATOR}" == "Chef Expeditor" ]]; then
    "$@"
  else
    echo "Build initiated by $BUILDKITE_BUILD_CREATOR"
    echo "Would have run: $*"
  fi
}

# Access the Fastly API token. Useful for purging caches.
fastly_token() {
    vault kv get \
          -field=token \
          account/static/fastly/eng-services-ops
}

latest_release_tag() {
  local repo="${1?repo argument required}"
  tag=$(curl --silent -L -H "X-GitHub-Api-Version: 2022-11-28" "https://api.github.com/repos/${repo}/releases/latest" | jq -r .tag_name)
  echo "${tag}"
}

install_hub() {
  # TODO: Create a Hab core plans pkg for this.
  # see https://github.com/habitat-sh/habitat/issues/7267
  local tag
  tag=$(latest_release_tag github/hub)
  tag_sans_v="${tag//v/}"
  url="https://github.com/github/hub/releases/download/${tag}/hub-linux-amd64-${tag_sans_v}.tgz"
  echo "--- :github: Installing hub version ${tag} to /bin/hub from ${url}"
  curl -L -O "${url}"
  tar xfz hub-linux-amd64-*.tgz
  cp -f hub-linux-amd64-*/bin/hub /bin
  chmod a+x /bin/hub
  rm -rf hub-linux-amd64*
}

# Push the current branch to the project origin
push_current_branch() {
  repo=$(git remote get-url origin | sed -rn  's/.+github\.com[\/\:](.*)\.git/\1/p')
  head=$(git rev-parse --abbrev-ref HEAD)

  if [ "$head" == "main" ]; then 
    echo "Error: Attempting to push to main!"
    exit 1
  fi

  git push "https://x-access-token:${GITHUB_TOKEN}@github.com/${repo}.git" "$head"
}


macos_install_bootstrap_package() {
    echo "--- Installing mac bootstrap package"
    # subscribe to releases: https://github.com/habitat-sh/release-engineering/issues/84
    bootstrap_package_version="$(cat MAC_BOOTSTRAPPER_VERSION)"
    bootstrap_package_name="mac-bootstrapper-${bootstrap_package_version}-1.x86_64"
    curl "https://packages.chef.io/files/stable/mac-bootstrapper/${bootstrap_package_version}/mac_os_x/10.15/${bootstrap_package_name}.dmg" -O
    sudo hdiutil attach "${bootstrap_package_name}.dmg"
    sudo installer -verbose -pkg "/Volumes/Habitat macOS Bootstrapper/${bootstrap_package_name}.pkg" -target /
    sudo hdiutil detach "/Volumes/Habitat macOS Bootstrapper"
    brew install wget
    export PATH=/opt/mac-bootstrapper/embedded/bin:/usr/local/bin:$PATH
    
}

macos_use_cert_file_from_linux_cacerts_package() {
    # BEGIN CERTIFICATE SHENANIGANS
    echo "--- Prepare cert file"
    # So, originally, we got this cert file from Homebrew's openssl
    # package. We currently need this because it gets baked into the
    # binaries we ship as part of how our API client library works. We may
    # be able to remove this in the near future, but for the time being,
    # we can just grab a cert file from the Linux core/cacerts Habitat
    # package.
    cacerts_scratch_dir="cacerts_scratch"
    rm -Rf "${cacerts_scratch_dir}"
    mkdir "${cacerts_scratch_dir}"
    hab pkg download core/cacerts \
        --target=x86_64-linux \
        --download-directory="${cacerts_scratch_dir}"
    cacerts_hart=$(find "${cacerts_scratch_dir}"/artifacts -type f -name 'core-cacerts-*-x86_64-linux.hart')

    # GNU tail, tar, from the mac-bootstrapper
    tail --lines=+6 "${cacerts_hart}" | \
        tar --extract \
            --verbose \
            --xz \
            --strip-components=8 \
            --wildcards "hab/pkgs/core/cacerts/*/*/ssl/certs"

    cert_file="cacert.pem"
    if [ ! -f "${cert_file}" ]; then
        echo "Couldn't extract ${cert_file} file from core/cacerts package!"
        exit 1
    fi
    export SSL_CERT_FILE
    SSL_CERT_FILE="$(pwd)/${cert_file}"

    # END CERTIFICATE SHENANIGANS
    ########################################################################
}

macos_sync_cache_signing_keys() {
    # We invoke hab-plan-build.sh directly via sudo, so we don't get the key management that studio provides.
    # Copy keys from the user account Habitat cache to the system Habitat cache so that they are present for root.
    sudo mkdir -p "${HAB_ROOT_PATH:-"/hab"}"/cache/keys
    sudo cp -r ~/.hab/cache/keys/* "${HAB_ROOT_PATH:-"/hab"}"/cache/keys/

}

macos_build() {
    local component="$1"
    echo "--- :habicat: Building $component"
    sudo -E bash \
         components/plan-build/bin/hab-plan-build.sh \
         "$component"
}
