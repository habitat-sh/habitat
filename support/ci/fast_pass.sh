#!/bin/bash
#
# Check to see what directories have been affected by a change. If directories
# have not been affected, exit 0
#
# Since we can only stop the build early by calling "exit" from within the
# .travis.yml in the `before_install`, we exit non-zero if we want the build to
# be skipped, so we can do `|| exit 0` in the YAML.

set -euo pipefail

echo "TAG: ${TRAVIS_TAG:=None}"
echo "VERSION: $(cat VERSION)"

if [ -n "${STEAM_ROLLER:-}" ]; then
  echo 'STEAM_ROLLER is set. Running tests unconditionally.'
elif [ -z "${AFFECTED_DIRS+x}" ]; then
  echo 'AFFECTED_DIRS is not set. Running tests unconditionally.'
elif [ -z "${AFFECTED_FILES+x}" ]; then
  echo 'AFFECTED_FILES is not set. Running tests unconditionally.'
elif [ "$(cat VERSION)" == "$TRAVIS_TAG" ]; then
  echo "This is a release tag. Congrats on the new  $TRAVIS_TAG release!!"
else
  # If $AFFECTED_DIRS and $AFFECTED_FILES is set, see if we have
  # any changes. To make this determination, we can always use the most recent merge
  # commit on the target branch, since Travis will have created one for us.
  CHANGED_FILES="$(support/ci/what_changed.sh)"

  echo_indented() {
    local x
    for x in "$@"; do
      echo "    $x"
    done
  }

  echo
  echo "Checking for changed files since last merge commit:"
  echo
  # shellcheck disable=2086
  echo_indented $CHANGED_FILES
  echo

  echo "Among the affected files:"
  echo
  # shellcheck disable=2086
  echo_indented $AFFECTED_FILES
  echo

  echo "And in the affected directories:"
  echo
  # shellcheck disable=2086
  echo_indented $AFFECTED_DIRS
  echo

  check_affected() {
    local changed_file=$1

    for dir in $AFFECTED_DIRS; do
      if [[ $changed_file = $dir* ]]; then
        echo "$changed_file in affected dir $dir"
        return 0
      fi
    done

    for affected_file in $AFFECTED_FILES; do
      if [ "$changed_file" = "$affected_file" ]; then
        echo "$changed_file is an affected file"
        return 0
      fi
    done

    return 1
  }

  CHANGED_AFFECTED_FILES=()
  for f in $CHANGED_FILES; do
    if check_affected "$f"; then
      CHANGED_AFFECTED_FILES+=($f)
    fi
  done

  if [ ${#CHANGED_AFFECTED_FILES[@]} -eq 0 ]; then
    echo "No files in affected directories or files have changed. Skipping CI run."
    exit 1
  fi
fi
