#!/bin/bash
#
# Check to see what directories have been affected by a change. If directories
# have not been affected, exit 0
#
# Since we can only stop the build early by calling "exit" from within the
# .travis.yml in the `before_install`, we exit non-zero if we want the build to
# be skipped, so we can do `|| exit 0` in the YAML.

echo "TAG: $TRAVIS_TAG"
echo "VERSION: $(cat VERSION)"

if [ -n "$STEAM_ROLLER" ]; then
  echo 'STEAM_ROLLER is set. Not exiting and running everything.'
elif [ -z "$AFFECTED_DIRS" ]; then
  # Don't do anything if $AFFECTED_DIRS is not set
  echo 'AFFECTED_DIRS is not set. Not exiting and running everything.'
elif [ "$(cat VERSION)" == "$TRAVIS_TAG" ]; then
  echo "This is a release tag. Congrats on the new  $TRAVIS_TAG release!!"
else
  # If $AFFECTED_DIRS (a "|" separated list of directories) is set, see if we have
  # any changes. To make this determination, we can always use the most recent merge
  # commit on the target branch, since Travis will have created one for us.
  CHANGED_FILES="$(support/ci/what_changed.sh)"

  echo
  echo "Checking for changed files since last merge commit:"
  echo
  echo "$CHANGED_FILES" | sed 's,^,    ,g'
  echo

  echo "In the affected directories:"
  echo
  echo "$AFFECTED_DIRS" | tr '|' '\n' | sed 's,^,    ,'
  echo

  echo "$CHANGED_FILES" | grep -qE "^($AFFECTED_DIRS)" || {
    echo "No files in affected directories have changed. Skipping CI run."
    exit 1
  }
fi
