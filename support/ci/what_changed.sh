#!/bin/bash

# Determine what changed since our last merge commit.
#
# Any time we merge code, we use a merge commit. If we're running this
# on a PR branch, we'll just grab the most recent merge commit as the
# base from which to perform our diffs.
#
# However, if we're building from master, then we'll actually be
# sitting on the last merge commit. In that case, we want to diff from
# the merge commit before that.

CURRENT_SHA="$(git rev-parse HEAD)"
LATEST_MERGE_COMMIT="$(git log --merges --max-count=1 --pretty=format:%H)"

if [ "${CURRENT_SHA}" == "${LATEST_MERGE_COMMIT}" ] ; then
    LATEST_MERGE_COMMIT="$(git log --merges --max-count=1 --skip=1 --pretty=format:%H)"
fi

git diff --name-only "${LATEST_MERGE_COMMIT}"
