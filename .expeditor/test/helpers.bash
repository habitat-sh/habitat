# Common helper functions and setup code for BATS-based testing.
#
# Every BATS test file should include the following line at the top;
# no exceptions:
#
#     load 'helpers'
#
# Happy testing!

# Load key BATS helper libraries
load '/test_helper/bats-support/load.bash'
load '/test_helper/bats-assert/load.bash'
load '/test_helper/bats-file/load.bash'

# A lot of our code calls to hab go through a `hab_binary` global
# variable, allowing us to use either the current stable release OR
# the release our pipeline is generating, depending on what stage
# we're at.
#
# For testing purposes, though, we don't particularly care, so we'll
# just use the binary that's available on the system.
declare -g hab_binary
hab_binary="hab"

# This is where things get mounted in the container.
REPO_ROOT="/test"

# Directory where various fixture files live
FIXTURES="${REPO_ROOT}/.expeditor/test/fixtures"

# Downloads a package (and any dependencies) to the test's temporary
# directory. Assumes $TEST_TEMP_DIR exists (generally done in a setup
# function).
#
# Returns the path to the downloaded hart file.
download_hart() {
    local ident=${1}
    local target=${2}
    hab pkg download "${ident}" --download-directory="${TEST_TEMP_DIR}" --target="${target}" > /dev/null 2>&1
    local ident_to_filename
    ident_to_filename="$(echo "${ident}" | tr "/" "-")"'*'"-${target}.hart"
    find "${TEST_TEMP_DIR}/artifacts" -type f -name "${ident_to_filename}"
}
