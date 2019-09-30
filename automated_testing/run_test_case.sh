#!/bin/bash

set -euo pipefail

# Every test case MUST define a docker-compose service with this
# name. This is the entrypoint for ALL tests.
test_service_name="tester"

testcase=${1}
defaults_dir="$(pwd)"
test_dir="${defaults_dir}/testcases"

if [ ! -d "${test_dir}/${testcase}" ]; then
    echo "No directory for ${testcase} found! Expected ${test_dir}/${testcase}"
    exit 1
fi

testcase_dir="${test_dir}/${testcase}"

declare -a compose_files
docker_compose="${defaults_dir}/docker-compose.yml"
compose_files+=("${docker_compose}")
echo "Docker Compose base file: ${docker_compose}"

if [ -f "${testcase_dir}/docker-compose.override.yml" ]; then
    override="${testcase_dir}/docker-compose.override.yml"
    echo "Using Override file: ${override}"
    compose_files+=("${override}")
fi
# TODO (CM): Is it an error to NOT define a test-specific override
# file? I think probably so...

# These are all standard docker-compose environment variables
export COMPOSE_PROJECT_NAME="habitat_integration_${testcase}"
export COMPOSE_PATH_SEPARATOR=":"
export COMPOSE_FILE
COMPOSE_FILE=$(IFS=${COMPOSE_PATH_SEPARATOR}; echo "${compose_files[*]}")

# These are assumed in our base docker-compose.yml file
export TESTCASE="${testcase}"
export TESTCASE_DIR="./testcases/${testcase}"

echo "Validating configuration..."
docker-compose config
echo "Valid!"

echo "Checking for presence of '${test_service_name}' service"
if docker-compose config --services | grep "${test_service_name}"; then
    echo "'${test_service_name}' service found!"
else
    echo "No service named '$test_service_name' is defined in any of the following files: ${COMPOSE_FILE}!"
    exit 1
fi

cleanup () {
    # TODO (CM): export logs on a per-service basis, taking into account
    # everything that is currently running for a given test case
    docker-compose logs
    docker-compose down
}

# The testing service is assumed to be something that needs to be
# built, as it is custom to a specific set of tests
docker-compose build "${test_service_name}"

# TODO (CM): capture the log output into a separate file
if docker-compose run "${test_service_name}"; then
    cleanup
else
    echo "OMG FAILURE!"
    cleanup
    exit 1
fi
