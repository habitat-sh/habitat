#!/usr/bin/env bash

set -euo pipefail

: "${BUILDKITE_BRANCH:?BUILDKITE_BRANCH is required}"
: "${BUILDKITE_BUILD_ID:?BUILDKITE_BUILD_ID is required}"
: "${BUILDKITE_COMMIT:?BUILDKITE_COMMIT is required}"
: "${GH_TOKEN:?GH_TOKEN with Actions read/write access is required}"

runtime_dir="$(mktemp -d)"
trap 'rm -rf "${runtime_dir}"' EXIT
source "$(dirname "${BASH_SOURCE[0]}")/gh_cli.sh"

if ! command -v gh >/dev/null 2>&1; then
    echo "--- Installing pinned GitHub CLI"
    install_gh_cli "${runtime_dir}"
fi

if ! command -v jq >/dev/null 2>&1; then
    echo "Required command 'jq' is not installed on this Buildkite agent" >&2
    exit 127
fi

repository="habitat-sh/habitat"
workflow="grype-scan-aarch64-windows.yml"
artifact_name="grype-scan-aarch64-windows"
report="${artifact_name}.txt"
request_id="${BUILDKITE_BUILD_ID}-grype-aarch64-windows-$(date -u +%Y%m%d%H%M%S)-${RANDOM}"
run_name="Grype scan aarch64-windows (${request_id})"

echo "--- Dispatching GitHub Actions Grype scan for aarch64-windows (${BUILDKITE_COMMIT})"
dispatch_command=(
    gh workflow run "${workflow}"
    --repo "${repository}"
    --ref "${BUILDKITE_BRANCH}"
    --raw-field "source_sha=${BUILDKITE_COMMIT}"
    --raw-field "request_id=${request_id}"
)
printf 'Command:'
printf ' %q' "${dispatch_command[@]}"
printf '\n'
"${dispatch_command[@]}"

run_id=""
for _ in $(seq 1 36); do
    run_id="$(
        gh run list \
            --repo "${repository}" \
            --workflow "${workflow}" \
            --event workflow_dispatch \
            --branch "${BUILDKITE_BRANCH}" \
            --limit 100 \
            --json databaseId,displayTitle \
            | jq -r --arg expected "${run_name}" \
                '[.[] | select(.displayTitle == $expected)] | first | .databaseId // empty'
    )"
    if [[ -n "${run_id}" ]]; then
        break
    fi
    sleep 5
done

if [[ -z "${run_id}" ]]; then
    echo "Could not find the dispatched GitHub Actions run '${run_name}'" >&2
    exit 1
fi

echo "--- Waiting for GitHub Actions Grype scan run ${run_id}"
deadline=$((SECONDS + 3600))
status=""
while (( SECONDS < deadline )); do
    run_state="$(
        gh api "repos/${repository}/actions/runs/${run_id}" \
            --jq '[.status, (.conclusion // "")] | @tsv'
    )"
    IFS=$'\t' read -r status conclusion <<<"${run_state}"

    if [[ "${status}" == "completed" ]]; then
        if [[ "${conclusion}" != "success" ]]; then
            echo "GitHub Actions Grype scan run ${run_id} finished with conclusion '${conclusion}'" >&2
            gh api "repos/${repository}/actions/runs/${run_id}" --jq .html_url >&2 || true
            exit 1
        fi
        break
    fi

    sleep 10
done

if [[ "${status}" != "completed" ]]; then
    echo "Timed out waiting for GitHub Actions Grype scan run ${run_id}" >&2
    gh run cancel "${run_id}" --repo "${repository}" || true
    exit 1
fi

artifact_dir="${runtime_dir}/artifacts"
mkdir -p "${artifact_dir}"
gh run download "${run_id}" \
    --repo "${repository}" \
    --name "${artifact_name}" \
    --dir "${artifact_dir}"

if [[ ! -s "${artifact_dir}/${report}" ]]; then
    echo "GitHub Actions run did not produce ${artifact_dir}/${report}" >&2
    exit 1
fi

cp "${artifact_dir}/${report}" "${report}"
echo "--- Grype report for aarch64-windows"
cat "${report}"
