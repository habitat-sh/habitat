#!/bin/bash
#
# # License and Copyright
#
# Copyright: Copyright (c) 2016 Chef Software, Inc.
# License: Apache License, Version 2.0
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

set -e

# The list of all specs to run after basic and env tests are run.
# WITHOUT .rb suffix.
all_specs=(crypto gossip)

# sadly, this is NOT a banner of a cat. But, with a pull request,
# YOU could make it so.
cat banner

# load in common test env vars
HAB=/bin/hab

export INSPEC_PACKAGE=core/inspec
export RUBY_PACKAGE=core/ruby
export RUBY_VERSION="2.3.0"
export BUNDLER_PACKAGE=core/bundler

install_package() {
    pkg_to_install=$1
    description=$2

    echo "» Installing ${description}"
    ${HAB} pkg install "${pkg_to_install}" >> ./logs/pkg_install.log 2>&1
    echo "★ Installed ${description}"
}

mkdir -p ./logs
echo "Installing Habitat testing packages..."

install_package ${INSPEC_PACKAGE} "Chef Inspec"
install_package ${BUNDLER_PACKAGE} "Bundler"


INSPEC_BUNDLE="$(hab pkg path $INSPEC_PACKAGE)/bundle"
GEM_HOME="${INSPEC_BUNDLE}/ruby/${RUBY_VERSION}"
GEM_PATH="$(hab pkg path ${RUBY_PACKAGE})/lib/ruby/gems/${RUBY_VERSION}:$(hab pkg path ${BUNDLER_PACKAGE}):${GEM_HOME}"
LD_LIBRARY_PATH="$(hab pkg path core/gcc-libs)/lib)"
export INSPEC_BUNDLE
export GEM_HOME
export GEM_PATH
export LD_LIBRARY_PATH

INSPEC="${HAB} pkg exec ${INSPEC_PACKAGE} inspec"
RSPEC="${HAB} pkg exec ${INSPEC_PACKAGE} rspec"


# This is required for rspec to pickup extra options via Inspec
SPEC_OPTS="--color --require spec_helper --format documentation"
export SPEC_OPTS


running_sups=$(pgrep hab-sup | wc -l)
if [ "$running_sups" -gt 0 ]; then
    echo "There are running Habitat supervisors, cannot continue testing"
    exit 1
fi

# TODO
if [ ! -z "$1" ]
  then
    echo "Running {$1}"
  else
    echo "Running all tests"
fi

####################################################################
# Run the tests!
####################################################################

echo "» Running tests"
test_start=$(date)
echo "☛ ${test_start}"

echo "» Checking for a clean test environment"
${INSPEC} exec ./hab_inspec/controls/clean_env.rb

echo "» Checking basic build/install/run functionality"
${RSPEC} ./spec/basic.rb

for s in "${all_specs[@]}"; do
    echo "» Running specs from ${s}"
    ${RSPEC} "./spec/${s}.rb"
done


####################################################################
# We're finished, report elapsed time
####################################################################
test_finish=$(date)
echo "☛ ${test_finish}"
echo "★ Finished"
