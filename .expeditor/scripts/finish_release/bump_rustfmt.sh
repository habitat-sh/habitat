#!/bin/bash

# This script attempts to find the latest installable nightly rustfmt.
# When found, it bumps the repo rustfmt version and submits a new PR
# with the new version pin and any code base formatting changes from the
# new version.
#
# Once an rustfmt nightly toolchain version is selected, we use the same
# toolchain version for the RUST_NIGHTLY_VERSION version pin.
# The RUST_NIGHTLY_VERSION file defines the toolchain used for cargo test.
# Since simply running `cargo test` does not require the installation
# of any additional rust components, using the same toolchain selected
# for rustfmt suffices.

# Assumptions:
# 1. This script runs on a x86_64-unknown-linux-gnu target
# 2. If rustfmt is installable on x86_64-unknown-linux-gnu, the other Tier1
#    platforms are assumed to also have a working version.

set -Eeuo pipefail

# shellcheck source=.expeditor/scripts/shared.sh
source .expeditor/scripts/shared.sh

TODAY=$(date '+%Y-%m-%d')
# pick out the date from a string like: nightly-2019-12-04
CURRENT_RUSTFMT_DATE=$(awk -F- '{print $2"-"$3"-"$4}' RUSTFMT_VERSION)
# convert to seconds for easier date comparison later
CURRENT_RUSTFMT_DATE_S=$(date '+%s' --date "${CURRENT_RUSTFMT_DATE}")
CHANNEL=nightly
# a maximum iterator value for searching elligible nightlies
MAX_ATTEMPTS=90

# if set, this will be the next version
export FOUND_VERSION

find_nightly_rustfmt() {
  # set minimal profile so only rustc, rust-std, and cargo components
  # are installed in addition to rustfmt
  echo "--- :habicat: Setting minimal profile"
  rustup set profile minimal

  for ((i=0; i<MAX_ATTEMPTS; i++)); do
    # begin searching nightly date versions using current date
    # as a starting point
    attempt_date=$(date '+%Y-%m-%d' --date "${TODAY} -${i} day")
    # use the conversion to seconds for easy date comparison
    attempt_date_s=$(date '+%s' --date "${attempt_date}")
    attempt_version="${CHANNEL}-${attempt_date}"
    echo "--- Trying Nightly Date Version: ${attempt_version}"
    if rustup toolchain install "${attempt_version}" --component rustfmt; then
      FOUND_VERSION=${attempt_version}
      break
    fi
    if [ "${attempt_date_s}" -le "${CURRENT_RUSTFMT_DATE_S}" ]; then
      # Exit early since there is no newer version available than what is
      # currently pinned in RUSTFMT_VERSION file.
      echo "--- No newer ${CHANNEL} versions found. Nothing to do."
      exit 0
    fi
  done
}

run_fmt_open_pr() {
  local version_underbar="${FOUND_VERSION//-/_}"

  # Do not change the branch name here without also changing
  # it in .expeditor/config.yml where a workload
  # subscription references the same name.
  readonly branch="expeditor/rustfmt_${version_underbar}"
  maybe_run git checkout -b "${branch}"

  # update the version pin files
  echo "${FOUND_VERSION}" | tee RUSTFMT_VERSION RUST_NIGHTLY_VERSION

  # sweep the codebase
  echo "--- :rust: Running new rustfmt"
  cargo +"$(< RUSTFMT_VERSION)" fmt --all

  git add --update
  maybe_run git commit --signoff --message "\"Bump nightly toolchain to ${FOUND_VERSION}\""

  # This script runs from a pipeline step, thus we do not
  # have access to expeditor's open_pull_request bash
  # action helper function.
  install_hub

  echo "--- :github: Creating PR"
  maybe_run hub pull-request --push --no-edit
}

install_rustup
find_nightly_rustfmt

if [ -z "${FOUND_VERSION}" ]; then
  # Something is likely wrong if we did not find an installable nightly version.
  echo "--- :thumbsdown: No candidate versions found within ${MAX_ATTEMPTS}!"
  exit 1
fi

echo "--- :thumbsup: New installable version of rustfmt found: ${FOUND_VERSION}"

run_fmt_open_pr
