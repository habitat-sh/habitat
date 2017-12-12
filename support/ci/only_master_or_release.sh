#!/bin/bash
#
# Use to stop a job early if we aren't on master or on a release tag.

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
    echo "Building a release for ${TRAVIS_TAG}"
elif [ "${TRAVIS_BRANCH}" == "master" ] && [ "${TRAVIS_PULL_REQUEST}" == "false" ] ; then
    echo "Building on the master branch"
else
    echo "Neither a release nor a master build; exiting!"
    exit 1
fi
