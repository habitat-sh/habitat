#!/bin/bash

set -Eeuo pipefail

# Assumptions:
# 1. This script runs from a bash action triggered from a workload subscription.
#    Therefore it assumes EXPEDITOR_SCM_SETTINGS_BRANCH ENV var exists.

# shellcheck source=.expeditor/scripts/shared.sh
source .expeditor/scripts/shared.sh

# https://expeditor.chef.io/docs/reference/workloads/#environment-variables
BRANCH=${EXPEDITOR_SCM_SETTINGS_BRANCH?required EXPEDITOR_SCM_SETTINGS_BRANCH ENV var not set}

export GITHUB_TOKEN
GITHUB_TOKEN="${GITHUB_TOKEN:-$(chef_ci_github_token)}"

pr_url_from_branch() {
  # assumes hub is being executed from git cloned repo workspace
  local branch="${1?branch argument required}"
  url=$(hub pr list --head "${branch}" --format %U)
  echo "${url}"
}

send_notification() {
  local url="${1?url argument required}"
  local message
  message=$(cat <<MSG
Hi @habitat-team, there is a rustfmt version bump PR ready for review! :jumping-habicat:

${url}

MSG
)

  echo "--- :slack: Notifying hab-team"
  # https://expeditor.chef.io/docs/reference/script/#post-slack-message
  maybe_run post_slack_message "hab-team" "\"${message}\""
}

# expeditor has no built-in way to fetch a PR URL, so we use hub
install_hub

echo "--- :github: Fetching PR URL of ${BRANCH} branch"
PR_URL=$(pr_url_from_branch "${BRANCH}")

# Even in the event that PR_URL is empty, we'll still send a notification.
send_notification "${PR_URL}"
