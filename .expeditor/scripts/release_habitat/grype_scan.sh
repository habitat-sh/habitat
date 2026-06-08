#!/bin/bash
# grype_scan.sh
#
# Installs the Habitat seed packages from the dev channel and runs a grype
# vulnerability scan on the Habitat root directory, writing Critical and High
# findings to a text file that is saved as a Buildkite artifact.
#
# The Habitat root differs by platform:
#   Linux (x86_64-linux, aarch64-linux)  →  /hab
#   macOS (aarch64-darwin)               →  /opt/hab
#
# Requires: BUILD_PKG_TARGET environment variable

set -euo pipefail

source .expeditor/scripts/release_habitat/shared.sh

export HAB_BLDR_URL="${PIPELINE_HAB_BLDR_URL}"
export HAB_LICENSE="accept-no-persist"

# ---------------------------------------------------------------------------
# Clear the Habitat root to ensure a clean install of only the dev packages
# ---------------------------------------------------------------------------
case "${BUILD_PKG_TARGET}" in
  aarch64-darwin) HAB_ROOT="/opt/hab" ;;
  *)              HAB_ROOT="/hab"     ;;
esac
echo "--- :broom: Clearing ${HAB_ROOT}"
sudo rm -rf "${HAB_ROOT}"

# ---------------------------------------------------------------------------
# Install hab
# ---------------------------------------------------------------------------
hab_binary=
curlbash_hab "${BUILD_PKG_TARGET}" "dev"

# Install jq-static from the base channel so jq is available for report generation
echo "--- :habicat: Installing core/jq-static"
"${hab_binary}" pkg install --binlink --force core/jq-static || true

# ---------------------------------------------------------------------------
# Install grype
# ---------------------------------------------------------------------------
echo "--- :mag: Installing grype"
curl -sSfL https://raw.githubusercontent.com/anchore/grype/main/install.sh \
  | sh -s -- -b /usr/local/bin
grype version

# ---------------------------------------------------------------------------
# Install seed packages from the dev channel
# ---------------------------------------------------------------------------
echo "--- :habicat: Installing seed packages from the dev channel"
SEED_PACKAGES=(
  chef/hab
  chef/hab-sup
  chef/hab-launcher
  chef/hab-studio
  chef/hab-pkg-export-tar
  chef/hab-pkg-export-container
)
for pkg in "${SEED_PACKAGES[@]}"; do
  echo "  Installing ${pkg}..."
  "${hab_binary}" pkg install --channel dev "${pkg}" \
    || echo "  WARNING: ${pkg} unavailable for ${BUILD_PKG_TARGET}, skipping"
done

# ---------------------------------------------------------------------------
# Run grype scan and write Critical/High findings to the report file
# ---------------------------------------------------------------------------
SCAN_DIR="${HAB_ROOT}"

REPORT="grype-scan-${BUILD_PKG_TARGET}.txt"

echo "--- :shield: Scanning ${SCAN_DIR} for Critical and High vulnerabilities"
RAW_JSON=$(grype "dir:${SCAN_DIR}" --output json 2>/dev/null)

CRITICAL=$(echo "${RAW_JSON}" | jq '[.matches[] | select(.vulnerability.severity == "Critical")] | length')
HIGH=$(echo "${RAW_JSON}" | jq '[.matches[] | select(.vulnerability.severity == "High")] | length')

{
  echo "Grype scan: ${BUILD_PKG_TARGET}"
  echo "Directory:  ${SCAN_DIR}"
  echo "Date:       $(date -u +"%Y-%m-%dT%H:%M:%SZ")"
  echo "Critical:   ${CRITICAL}"
  echo "High:       ${HIGH}"
  echo ""

  if [[ "${CRITICAL}" -eq 0 && "${HIGH}" -eq 0 ]]; then
    echo "No Critical or High vulnerabilities found."
  else
    printf "%-10s %-22s %-35s %-18s %-20s %s\n" "SEVERITY" "CVE" "PACKAGE" "VERSION" "FIX" "HAB_PKG_PATH"
    printf "%-10s %-22s %-35s %-18s %-20s %s\n" "--------" "---" "-------" "-------" "---" "------------"
    echo "${RAW_JSON}" | jq -r '
      .matches[]
      | select(.vulnerability.severity == "Critical" or .vulnerability.severity == "High")
      | [
          .vulnerability.severity,
          .vulnerability.id,
          .artifact.name,
          .artifact.version,
          (.vulnerability.fix.versions // [] | if length > 0 then join(", ") else "none" end),
          ((.artifact.locations // [])
            | map(.path // .realPath // .accessPath // "")
            | map(select(. != ""))
            | if length > 0 then
                .[0] |
                if test("(?:(?:/opt)?/hab/)?pkgs/[^/]+/[^/]+/[^/]+/[^/]+") then
                  match("((?:(?:/opt)?/hab/)?pkgs/[^/]+/[^/]+/[^/]+/[^/]+)").captures[0].string
                else . end
              else "no-location" end)
        ]
      | @tsv
    ' | sort | while IFS=$'\t' read -r sev cve pkg ver fix habpath; do
        printf "%-10s %-22s %-35s %-18s %-20s %s\n" "${sev}" "${cve}" "${pkg}" "${ver}" "${fix}" "${habpath}"
      done
  fi
} > "${REPORT}"

echo "Results saved to ${REPORT}"
cat "${REPORT}"
