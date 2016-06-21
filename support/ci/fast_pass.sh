#!/bin/bash
#
# Check to see what directories have been affected by a change. If directories
# have not been affected, exit 0
#
# Since we can only stop the build early by calling "exit" from within the
# .travis.yml in the `before_install`, we exit non-zero if we want the build to
# be skipped, so we can do `|| exit 0` in the YAML.

# Don't do anything if $AFFECTED_DIRS is not set
if [ -z "$AFFECTED_DIRS" ]; then
  echo 'AFFECTED_DIRS is not set. Not exiting and running everything.'
# If $AFFECTED_DIRS (a "|" separated list of directories) is set, see if we have
# any changes
else
  git diff --name-only "$TRAVIS_COMMIT_RANGE" | grep -qE "^($AFFECTED_DIRS)" || {
    echo "No files in $AFFECTED_DIRS have changed. Skipping CI run."
    exit 1
  }
fi
