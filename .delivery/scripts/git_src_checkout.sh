#!/bin/bash
# We should only use this when we're in delivery, run from a build
# cookbook that sets the GITHUB_DEPLOY_KEY and DELIVERY_GIT_SHASUM.
#
# We're going to use the ssh wrapper so we can clone the private repo
# from github. This is dropped off by the Dockerfile COPY.
export GIT_SSH=/usr/local/bin/ssh_wrapper.sh

# We need the SSH key.
echo "$0: Writing Github deploy key to ~/.ssh/id_rsa_bldr_github"
mkdir -p ~/.ssh
echo "${GITHUB_DEPLOY_KEY}" > ~/.ssh/id_rsa_bldr_github
chmod 0600 ~/.ssh/id_rsa_bldr_github

echo "$0: Checking for bldr plans in /src/plans"
if [ -d /src/plans ]
then
    echo "$0: /src exists, attempt to fetch from Github"
    cd /src
    git fetch || true
else
    echo "$0: /src does not exist, clone from Github"
    (cd / && git clone git@github.com:chef/bldr.git /src)
fi

echo "$0: Set working tree to '${DELIVERY_GIT_SHASUM}'"
(git rev-parse HEAD | grep -q "${DELIVERY_GIT_SHASUM}") || git checkout ${DELIVERY_GIT_SHASUM} || exit 1

if [ -f /src/plans/bash/plan.sh ]
then
    echo "$0: /src/plans/bash/plan.sh exists, success!"
    exit 0
else
    echo "$0: /src/plans/bash/plan.sh does not exist, failure!"
    exit 1
fi

exit 0
