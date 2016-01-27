#!/bin/bash
# We should only use this when we're in delivery, run from a build
# cookbook that sets the GITHUB_DEPLOY_KEY and DELIVERY_GIT_SHASUM.
#
# We're going to use the ssh wrapper so we can clone the private repo
# from github. This should be dropped off by the Dockerfile.
GIT_SSH=/usr/local/bin/ssh_wrapper.sh

# We need the SSH key.
mkdir -p ~/.ssh
echo "${GITHUB_DEPLOY_KEY}" > ~/.ssh/id_rsa_bldr_github
chmod 0600 ~/.ssh/id_rsa_bldr_github

if [ -d /src/plans ]
then
    cd /src
    git pull || true
else
    (cd / && git clone git@github.com:chef/bldr.git /src)
fi

# TODO: REMOVE DEBUGGING
echo "DEBUG OUTPUT"
whoami
env | egrep 'GIT|DOCK|DELIV'
echo $PWD
ls -a $PWD /src/plans
(cd /src ; git status)
echo "END DEBUG OUTPUT"

(git rev-parse HEAD | grep -q "${DELIVERY_GIT_SHASUM}") || git checkout ${DELIVERY_GIT_SHASUM} || exit 1
exit 0
