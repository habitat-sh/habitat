#!/usr/bin/env bash

set -euo pipefail

test_name="${1:?test name argument required}"
case "${test_name}" in
    test_studio_version | test_studio_profile | test_studio_can_build_packages \
        | test_studio_can_build_packages_with_pkg_version_function \
        | test_studio_can_build_scaffolded_package | test_studio_with_ssl_cert_file_envvar_set \
        | test_ssl_certificate_loading | test_pkg_install | test_pkg_uninstall_hook | test_hab_plan_cli) ;;
    *)
        echo "Unsupported aarch64-windows e2e test: ${test_name}" >&2
        exit 2
        ;;
esac

: "${BUILDKITE_BRANCH:?BUILDKITE_BRANCH is required}"
: "${BUILDKITE_BUILD_ID:?BUILDKITE_BUILD_ID is required}"
: "${BUILDKITE_COMMIT:?BUILDKITE_COMMIT is required}"
: "${GH_TOKEN:?GH_TOKEN with Actions read/write access is required}"

runtime_dir="$(mktemp -d)"
run_id=""
finished=false
repository="habitat-sh/habitat"

cleanup() {
    # Do not leave tests running if the Buildkite job is canceled or times out.
    if [[ -n "${run_id}" && "${finished}" != true ]]; then
        gh run cancel "${run_id}" --repo "${repository}" || true
    fi
    rm -rf "${runtime_dir}"
}
trap cleanup EXIT
trap 'exit 130' INT
trap 'exit 143' TERM

source "$(dirname "${BASH_SOURCE[0]}")/../release_habitat/gh_cli.sh"
if ! command -v gh >/dev/null 2>&1; then
    echo "--- Installing pinned GitHub CLI"
    install_gh_cli "${runtime_dir}"
fi
if ! command -v jq >/dev/null 2>&1; then
    echo "Required command 'jq' is not installed on this Buildkite agent" >&2
    exit 127
fi

workflow="e2e-aarch64-windows.yml"
request_id="${BUILDKITE_BUILD_ID}-${test_name//_/-}-$(date -u +%Y%m%d%H%M%S)-${RANDOM}"
run_name="Habitat e2e ${test_name} (${request_id})"

echo "--- Dispatching GitHub Actions ${test_name} for aarch64-windows (${BUILDKITE_COMMIT})"
gh workflow run "${workflow}" \
    --repo "${repository}" \
    --ref "${BUILDKITE_BRANCH}" \
    --raw-field "test_name=${test_name}" \
    --raw-field "source_sha=${BUILDKITE_COMMIT}" \
    --raw-field "request_id=${request_id}"

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

echo "--- Waiting for https://github.com/${repository}/actions/runs/${run_id}"
# Allow queueing time in addition to the Windows e2e job's 30-minute timeout.
deadline=$((SECONDS + 3600))
while (( SECONDS < deadline )); do
    run_state="$(
        gh api "repos/${repository}/actions/runs/${run_id}" \
            --jq '[.status, (.conclusion // "")] | @tsv'
    )"
    IFS=$'\t' read -r status conclusion <<<"${run_state}"
    if [[ "${status}" == "completed" ]]; then
        finished=true
        break
    fi
    sleep 10
done

if [[ "${finished}" != true ]]; then
    echo "Timed out waiting for GitHub Actions e2e run ${run_id}" >&2
    exit 1
fi

# Keep logs even when a test fails, so they are available in Buildkite.
artifact_dir="e2e-aarch64-windows/${request_id}"
mkdir -p "${artifact_dir}"
download_status=0
gh run download "${run_id}" --repo "${repository}" --dir "${artifact_dir}" || download_status=$?

log_status=0
gh run view "${run_id}" --repo "${repository}" --log \
    > "${artifact_dir}/github-actions.log" || log_status=$?

echo "--- Test output for ${test_name} (aarch64-windows)"
test_step="Run end-to-end test against dev packages"
# gh prefixes each log line with tab-separated job and step names. Keep the
# timestamp and message, including any tabs within the message itself.
awk -F '\t' -v step="${test_step}" '
    $2 == step { sub(/^[^\t]*\t[^\t]*\t/, ""); print }
' "${artifact_dir}/github-actions.log"
echo "Full GitHub Actions log: ${artifact_dir}/github-actions.log"

if [[ "${conclusion}" != "success" ]]; then
    echo "--- Failed GitHub Actions steps (excluding test output above)"
    gh run view "${run_id}" --repo "${repository}" --log-failed \
        | awk -F '\t' -v step="${test_step}" '
            $2 != step { sub(/^[^\t]*\t[^\t]*\t/, ""); print }
        ' || true
    echo "GitHub Actions e2e run ${run_id} finished with conclusion '${conclusion}'" >&2
    exit 1
fi
if [[ "${download_status}" -ne 0 ]]; then
    exit "${download_status}"
fi
exit "${log_status}"
