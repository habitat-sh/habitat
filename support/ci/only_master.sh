#!/bin/bash
#
# Use to stop a job early if:
#   a) we are NOT on the `master` branch, or
#   b) we ARE on a release tag.
#
# We used to run releases out of TravisCI, and that was based on
# running off of a tag. Now, however, we run releases out of
# Buildkite, so we should never build on tags in Travis (otherwise
# we'll be doing twice the work, and stepping on Buildkite's
# metaphorical toes).
#
# We don't currently have daily CI tasks set up in Buildkite, so we
# can still take advantage of Travis to do builds on master following
# PR merges.

echo "Habitat VERSION File Contents: $(cat VERSION)"
echo "TRAVIS_TAG: ${TRAVIS_TAG}"
echo "TRAVIS_BRANCH: ${TRAVIS_BRANCH}"
echo "TRAVIS_PULL_REQUEST_BRANCH: ${TRAVIS_PULL_REQUEST_BRANCH}"

# From https://docs.travis-ci.com/user/environment-variables
#     TRAVIS_BRANCH:
#       for push builds, or builds not triggered by a pull request, this is the name of the branch.
#       for builds triggered by a pull request this is the name of the branch targeted by the pull request.
#       for builds triggered by a tag, this is the same as the name of the tag (TRAVIS_TAG).
#     TRAVIS_PULL_REQUEST:
#       The pull request number if the current job is a pull request, “false” if it’s not a pull request.
if [ "${TRAVIS_TAG}" == "$(cat VERSION)" ]; then
    echo "Looks like a release tag; not building in Travis. Please check out Buildkite instead."
    exit 1
elif [ "${TRAVIS_BRANCH}" == "master" ] && [ "${TRAVIS_PULL_REQUEST}" == "false" ] ; then
    echo "Building on the master branch"
else
    echo "We're not on master, or Buildkite is running our release; exiting!"
    exit 1
fi
