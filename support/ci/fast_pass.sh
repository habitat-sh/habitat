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
  # any changes. To figure this out, we need to compare the contents of all commits
  # since the last merge commit (excluding the most recent one, if it is one).

  LATEST_MERGE_COMMIT=$(git log --merges --max-count=1 --pretty=format:%H)

  if [ "$TRAVIS_COMMIT" == "$LATEST_MERGE_COMMIT" ]; then
    COMMIT_RANGE="$TRAVIS_COMMIT"
  else
    COMMIT_RANGE="$TRAVIS_COMMIT...$LATEST_MERGE_COMMIT"
  fi

  CHANGED_FILES=$(git log -m --max-count=1 --name-only --pretty=format: "$COMMIT_RANGE")

  echo "LATEST_MERGE_COMMIT: $LATEST_MERGE_COMMIT"
  echo "COMMIT_RANGE: $COMMIT_RANGE"
  echo "CHANGED_FILES: $CHANGED_FILES"

  echo "$CHANGED_FILES" | grep -qE "^($AFFECTED_DIRS)" || {
    echo "No files in $AFFECTED_DIRS have changed. Skipping CI run."
    exit 1
  }
fi
