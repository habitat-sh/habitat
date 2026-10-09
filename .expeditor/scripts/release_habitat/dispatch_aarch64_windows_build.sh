#!/usr/bin/env bash

set -euo pipefail

component="${1:?component argument required}"
case "${component}" in
    hab | plan-build-ps1 | studio) ;;
    *)
        echo "Unsupported aarch64-windows release component: ${component}" >&2
        exit 2
        ;;
esac

: "${BUILDKITE_BRANCH:?BUILDKITE_BRANCH is required}"
: "${BUILDKITE_BUILD_ID:?BUILDKITE_BUILD_ID is required}"
: "${BUILDKITE_COMMIT:?BUILDKITE_COMMIT is required}"
: "${GH_TOKEN:?GH_TOKEN with Actions read/write access is required}"

for command in gh jq buildkite-agent; do
    if ! command -v "${command}" >/dev/null 2>&1; then
        echo "Required command '${command}' is not installed on this Buildkite agent" >&2
        exit 127
    fi
done

repository="habitat-sh/habitat"
workflow="release-aarch64-windows.yml"
release_channel="habitat-release-${BUILDKITE_BUILD_ID}"
request_id="${BUILDKITE_BUILD_ID}-${component}-$(date -u +%Y%m%d%H%M%S)-${RANDOM}"
run_name="Habitat release ${component} (${request_id})"

echo "--- Dispatching GitHub Actions release build for ${component} (${BUILDKITE_COMMIT})"
gh workflow run "${workflow}" \
    --repo "${repository}" \
    --ref "${BUILDKITE_BRANCH}" \
    --raw-field "component=${component}" \
    --raw-field "release_channel=${release_channel}" \
    --raw-field "source_sha=${BUILDKITE_COMMIT}" \
    --raw-field "request_id=${request_id}"

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

echo "--- Waiting for GitHub Actions run ${run_id}"
deadline=$((SECONDS + 3600))
while (( SECONDS < deadline )); do
    run_state="$(
        gh api "repos/${repository}/actions/runs/${run_id}" \
            --jq '[.status, (.conclusion // "")] | @tsv'
    )"
    IFS=$'\t' read -r status conclusion <<<"${run_state}"

    if [[ "${status}" == "completed" ]]; then
        if [[ "${conclusion}" != "success" ]]; then
            echo "GitHub Actions run ${run_id} finished with conclusion '${conclusion}'" >&2
            gh api "repos/${repository}/actions/runs/${run_id}" --jq .html_url >&2 || true
            exit 1
        fi
        break
    fi

    sleep 10
done

if [[ "${status:-}" != "completed" ]]; then
    echo "Timed out waiting for GitHub Actions run ${run_id}" >&2
    exit 1
fi

artifact_dir="$(mktemp -d)"
trap 'rm -rf "${artifact_dir}"' EXIT
gh run download "${run_id}" \
    --repo "${repository}" \
    --name release-build-result \
    --dir "${artifact_dir}"

result_file="${artifact_dir}/release-build-result.json"
if [[ ! -s "${result_file}" ]]; then
    echo "GitHub Actions run did not produce ${result_file}" >&2
    exit 1
fi

result_component="$(jq -er '.component | strings' "${result_file}")"
result_ident="$(jq -er '.ident | select(type == "string" and startswith("chef/"))' "${result_file}")"
result_target="$(jq -er '.target | strings' "${result_file}")"
result_channel="$(jq -er '.channel | strings' "${result_file}")"

if [[ "${result_component}" != "${component}" \
      || "${result_target}" != "aarch64-windows" \
      || "${result_channel}" != "${release_channel}" ]]; then
    echo "Unexpected release build result: $(cat "${result_file}")" >&2
    exit 1
fi

echo "--- Recording ${result_ident} (aarch64-windows) in Buildkite metadata"
buildkite-agent meta-data set "${result_ident}-aarch64-windows" true
buildkite-agent annotate --append --context 'release-manifest' \
    "<br>* ${result_ident} (aarch64-windows)"
