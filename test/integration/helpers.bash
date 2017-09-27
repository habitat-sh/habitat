#!/usr/bin/env bats

# Common helper functions and setup code for Habitat BATS-based
# testing.
#
# Every BATS test file should include the following line at the top;
# no exceptions:
#
#     load 'helpers'
#
# Happy testing!

# Load key BATS helper libraries
load 'test_helper/bats-support/load'
load 'test_helper/bats-assert/load'
load 'test_helper/bats-file/load'

# Point to our local hab binaries!
if [ -z "${HAB_BIN_DIR}" ]; then
    echo "Must set HAB_BIN_DIR variable to a location that contains hab, hab-sup, and hab-launch binaries!"
    exit 1
fi
hab="${HAB_BIN_DIR}/hab"
export HAB_SUP_BINARY="${HAB_BIN_DIR}/hab-sup"
export HAB_LAUNCH_BINARY="${HAB_BIN_DIR}/hab-launch"

# Ensure required utilities are present
find_if_exists() {
    command -v ${1} || { log "Required utility '${1}' cannot be found!  Aborting."; exit 1; }
}
awk=$(find_if_exists awk)
curl=$(find_if_exists curl)
jq=$(find_if_exists jq)

# Custom Assertions
########################################################################

assert_spec_exists_for() {
    local service_name=${1}
    assert_file_exist $(spec_file_for ${service_name})
}

# Given a fully-qualified package identifer, assert that the package has been
# installed.
#
# This means that:
# - the package has been cached into /hab/cache/artifacts
# - the public signing key for the package has been cached in /hab/cache/keys
# - the package has been unpacked into the appropriate directory in /hab/pkgs/
#
# Note that dependencies of the package are NOT checked in this
# assertion. For that, see `assert_package_and_deps_installed` below.
assert_package_installed() {
    local ident=${1}
    local cached_hart_file=$(cached_artifact_for "${ident}")

    assert_file_exist "${cached_hart_file}"
    assert_file_exist $(cached_signing_key_for "${cached_hart_file}")
    assert_file_exist $(installation_directory_for "${ident}")
}

# Given a fully-qualified package identifer, assert that it and all
# its transitive dependencies have been completely installed.
assert_package_and_deps_installed() {
    ident=${1}
    assert_package_installed "${ident}"

    tdeps_file="/hab/pkgs/${ident}/TDEPS"
    if [ -e "${tdeps_file}" ]; then
        for dep in $(cat "${tdeps_file}"); do
            assert_package_installed "${dep}"
        done
    fi
}

# Given a fully-qualified identifier, verify that that service is
# running and that all associated service files have been created.
assert_service_running() {
    local ident=${1}
    declare -a parsed
    IFS='/' read -r -a parsed <<< "${ident}"

    local service_name="${parsed[1]}"
    echo "ASR: ${service_name}" >&2

    assert_file_exist "$(spec_file_for ${service_name})"
    assert_file_exist "$(service_directory_for ${service_name})"
    assert_equal $(current_running_version_for "${service_name}") "${ident}"
}

# Useful Setup / Teardown Functions
########################################################################

# Some tests start up a Habitat supervisor in the background. Call
# this in a `teardown` function to ensure it is stopped before the
# next test.a
stop_supervisor() {
    pkill hab-launch || true
}

# Ensure a clean slate in `/hab` for each test
reset_hab_root() {
    empty_artifact_cache
    empty_key_cache
    remove_all_services
    remove_installed_packages
    reset_supervisor
}

empty_artifact_cache() {
    rm -Rf /hab/cache/artifacts/*
}

empty_key_cache() {
    rm -Rf /hab/cache/keys/*
}

remove_all_services() {
    rm -Rf /hab/svc/*
}

remove_installed_packages() {
    rm -Rf /hab/pkgs/*
}

reset_supervisor() {
    rm -Rf /hab/sup/*
}

# Helper Functions
########################################################################

# Stop a test with the given message output to standard error for
# debugging.
fail_with_msg() {
    echo ${1} >&2
    exit 1
}

# Stolen from
# https://github.com/docker/swarm/blob/master/test/integration/helpers.bash
retry() {
    local attempts=$1
    shift
    local delay=$1
    shift
    local i

    for ((i=0; i < attempts; i++)); do
        run "$@"
        if [[ "$status" -eq 0 ]] ; then
            return 0
        fi
        sleep $delay
    done

    echo "Command \"$@\" failed $attempts times. Output: $output"
    false
}

# Run the given command in the background. Useful for starting a
# supervisor and then moving on with commands that exercise it for the
# test.
#
# Ensures that BATS' handling of file handles is taken into account;
# see
# https://github.com/sstephenson/bats/issues/80#issuecomment-174101686
# for details.
#
# (Also, it's really easy to leave the `&` off when you're writing
# and editing tests. This makes things _super_ explicit.)
background() {
    "$@" 3>- &
}

# Returns the PID of the given service.
#
# Fails if the pidfile does not exist. Does not test that the PID is
# alive.
pid_of_service() {
    local service_name="${1}"
    local pidfile="/hab/svc/${service_name}/PID"
    if [ -e "${pidfile}" ]; then
        cat "${pidfile}"
    else
        false
    fi
}

service_is_alive() {
    local service_name="${1}"
    local pid=$(pid_of_service "${service_name}")
    ps -p "${pid}" > /dev/null 2>&1
}

# Checks once a second to see if the Habitat-supervised service
# has is running yet.
wait_for_service_to_run() {
    local service_name=${1}
    retry 30 1 service_is_alive "${service_name}" #bash -c "[ -e /hab/svc/${service_name}/PID ]"
}

pid_has_changed() {
    local service_name="${1}"
    local original_pid="${2}"
    local current_pid=$(pid_of_service "${service_name}")
    [ "${current_pid}" -ne "${original_pid}" ]
}

wait_for_service_to_restart() {
    local service_name=${1}
    local original_pid=${2}
    retry 30 1 pid_has_changed "${service_name}" "${original_pid}"
}

current_running_version_for() {
    service_name=${1}
    member_id=$(cat /hab/sup/default/MEMBER_ID)

    ${jq} -r '.census_groups."redis.default".population."'${member_id}'".pkg | (.origin + "/" +.name + "/" + .version + "/" + .release)' /hab/sup/default/data/census.dat
}

# Given a package identifier and a channel name, query Builder to discover the
# latest release of that package.
#
# Arguments:
#    ${1}: a Habitat package identifier, i.e.
#            - origin/package
#            - origin/package/version
#          Fully-qualified identifiers aren't supported in
#          this function; there's no such thing as a "latest" version of a
#          fully-qualified release, since those are immutable.

#    ${2}: a channel name
#
# Examples:
#
#    latest_from_builder core/redis stable
#    # => core/redis/x.x.x/nnnnnnnnnnnnnn from `stable` channel
#
#    latest_from_builder core/redis/2.3.4 stable
#    # => core/redis/2.3.4/nnnnnnnnnnnnnn from `stable` channel
#
#    latest_from_builder core/redis unstable
#    # => same, but from the 'unstable' channel
#
latest_from_builder() {
    # Parse out the individual components of the package identifier.
    local ident=${1}
    declare -a parsed
    IFS='/' read -r -a parsed <<< "${ident}"

    local channel=${2}
    local url
    local origin
    local package_name
    local version

    url_base="https://bldr.habitat.sh/v1/depot"

    # Generate an appropriate URL based on our input
    case "${#parsed[@]}" in
        2)
            origin="${parsed[0]}"
            package_name="${parsed[1]}"
            url="${url_base}/channels/${origin}/${channel}/pkgs/${package_name}/latest"
            ;;
        3)
            origin="${parsed[0]}"
            package_name="${parsed[1]}"
            version="${parsed[2]}"
            url="${url_base}/channels/${origin}/${channel}/pkgs/${package_name}/${version}/latest"
            ;;
        4)
            fail_with_msg "Fully-qualified identifiers are not supported in this function: '${ident}'"
            ;;
        *)
            fail_with_msg "Invalid package identifier given: '${ident}'"
    esac
    echo ${url} >&2

    # TODO (CM): If there is no version in the specified channel, this is
    # going to return nothing... should we fail?

    ${curl} --silent "${url}" | \
        ${jq} -r '.ident | (.origin + "/" +.name + "/" + .version + "/" + .release)'
}

# Extract the signing key name from the header of the given `.hart`
# file.
signing_key_name() {
    local hart_archive=${1}
    # The key name is the second line of the file
    ${awk} 'NR==2' ${hart_archive}
}

# Given a fully-qualified package identifier, return the path to the
# cached `.hart` file on disk.
cached_artifact_for() {
    local ident=${1}
    parsed=()
    IFS='/' read -r -a parsed <<< "${ident}"

    local origin="${parsed[0]}"
    local package_name="${parsed[1]}"
    local version="${parsed[2]}"
    local release="${parsed[3]}"

    # Hard-coding this for now
    local platform="x86_64-linux"

    local file="/hab/cache/artifacts/${origin}-${package_name}-${version}-${release}-${platform}.hart"

    echo "${file}"
}

# Return the path to the cached signing key for a given hart file
cached_signing_key_for() {
    local hart_file=${1}
    local key_name=$(signing_key_name "${hart_file}")

    echo "/hab/cache/keys/${key_name}.pub"
}

# Given a fully-qualified identifier, returns the path to where the
# package would be unpacked on disk.
installation_directory_for() {
    local ident=${1}
    echo "/hab/pkgs/${ident}"
}

# Given a fully-qualified identifier, use `hab` to retrieve the hart
# file for it and place it into `$BATS_TMPDIR`, returning the full
# path to the hart file.
download_hart_for() {
    ident=${1}

    run ${hab} pkg install "${ident}"
    assert_success

    cached_artifact=$(cached_artifact_for "${ident}")
    file_name=$(basename "${cached_artifact}")

    cp ${cached_artifact} "${BATS_TMPDIR}"
    echo "${BATS_TMPDIR}/${file_name}"
}

# Returns the directory that the named service has should have.
service_directory_for() {
    local service_name=${1}
    echo "/hab/svc/${service_name}"
}

# Returns the path to the named service's spec file.
spec_file_for() {
    local service_name=${1}
    echo "SFF: ${service_name}" >&2
    echo "/hab/sup/default/specs/${service_name}.spec"
}
