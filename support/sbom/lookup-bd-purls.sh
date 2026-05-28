#!/usr/bin/env bash
# Look up BlackDuck KB PURLs for each component in habitat-core-deps.cyclonedx.json
# and write an updated version with BD's canonical PURLs.
#
# BD matches SBOM components by PURL. Using BD's own PURLs ensures each
# component maps to the correct existing KB entry instead of creating duplicates.
#
# Usage:
#   export BD_URL=https://progresssoftware.app.blackduck.com
#   export BD_TOKEN=<your-api-token>
#   bash support/sbom/lookup-bd-purls.sh
#
#   Writes:  support/sbom/habitat-core-deps.cyclonedx.json  (in-place update)
#   Preview: bash support/sbom/lookup-bd-purls.sh --dry-run
#
# Requires: curl, jq

set -euo pipefail

BD_URL="${BD_URL:?Set BD_URL to your BlackDuck instance URL (no trailing slash)}"
BD_TOKEN="${BD_TOKEN:?Set BD_TOKEN to your BlackDuck API token}"
INPUT="support/sbom/habitat-core-deps.cyclonedx.json"
DRY_RUN=false
[[ "${1:-}" == "--dry-run" ]] && DRY_RUN=true

# ── Authenticate ────────────────────────────────────────────────────────────
echo "Authenticating with BlackDuck..." >&2
BEARER=$(curl -sSf -X POST "${BD_URL}/api/tokens/authenticate" \
  -H "Authorization: token ${BD_TOKEN}" \
  -H "Accept: application/vnd.blackducksoftware.user-4+json" \
  | jq -r '.bearerToken')
if [ -z "$BEARER" ] || [ "$BEARER" = "null" ]; then
  echo "ERROR: Authentication failed." >&2; exit 1
fi
echo "Authenticated." >&2

# ── Helper: look up a component version in BD and return its PURL ───────────
# Returns one of:
#   <purl>             — found
#   NOT_FOUND_COMPONENT — component not in BD KB
#   NOT_FOUND_VERSION   — component found but version not in BD KB
#   CURL_ERROR          — network/auth failure (curl non-zero exit); caller should abort
lookup_purl() {
  local name="$1" version="$2"
  # URL-encode name and version for use in query strings
  local bd_name_encoded bd_version_encoded
  bd_name_encoded=$(printf '%s' "$name" | jq -sRr @uri)
  bd_version_encoded=$(printf '%s' "$version" | jq -sRr @uri)

  # Search for the component by name (BD does substring match, filter exact)
  local search
  if ! search=$(curl -sSf \
    "${BD_URL}/api/components?q=name:${bd_name_encoded}&limit=20" \
    -H "Authorization: Bearer ${BEARER}" \
    -H "Accept: application/vnd.blackducksoftware.internal-1+json"); then
    echo "CURL_ERROR" ; return
  fi

  local comp_href
  comp_href=$(printf '%s' "$search" | jq -r \
    --arg n "$name" \
    '.items // [] | .[] | select(.name == $n) | ._meta.href' | head -1)

  if [ -z "$comp_href" ] || [ "$comp_href" = "null" ]; then
    echo "NOT_FOUND_COMPONENT" ; return
  fi

  # Look for the specific version
  local ver_result
  if ! ver_result=$(curl -sSf \
    "${comp_href}/versions?q=versionName:${bd_version_encoded}&limit=10" \
    -H "Authorization: Bearer ${BEARER}" \
    -H "Accept: application/vnd.blackducksoftware.component-detail-5+json"); then
    echo "CURL_ERROR" ; return
  fi

  # BD stores the PURL in packageUrl; fall back to externalId
  local purl
  purl=$(printf '%s' "$ver_result" | jq -r \
    --arg v "$version" \
    '.items // [] | .[] | select(.versionName == $v) | .packageUrl // .externalId // ""' \
    | head -1)

  # If version-level PURL is empty, try the component-level packageUrl
  if [ -z "$purl" ] || [ "$purl" = "null" ]; then
    purl=$(printf '%s' "$search" | jq -r \
      --arg n "$name" \
      '.items // [] | .[] | select(.name == $n) | .packageUrl // ""' | head -1)
  fi

  if [ -z "$purl" ] || [ "$purl" = "null" ]; then
    echo "NOT_FOUND_VERSION" ; return
  fi

  echo "$purl"
}

# ── Process each component ───────────────────────────────────────────────────
echo "" >&2
echo "Looking up PURLs for $(jq '.components | length' "$INPUT") components..." >&2
echo "" >&2

# Read all components, update PURLs, collect results
UPDATED_COMPONENTS=$(jq -c '.components[]' "$INPUT" | while IFS= read -r component; do
  name=$(echo    "$component" | jq -r '.name')
  version=$(echo "$component" | jq -r '.version')
  orig_purl=$(echo "$component" | jq -r '.purl // ""')

  result=$(lookup_purl "$name" "$version")

  case "$result" in
    CURL_ERROR)
      printf "  %-30s %-15s  ✗  BD API request failed (network/auth error)\n" "$name" "$version" >&2
      exit 1
      ;;
    NOT_FOUND_COMPONENT)
      printf "  %-30s %-15s  ⚠  component not in BD KB\n" "$name" "$version" >&2
      echo "$component"
      ;;
    NOT_FOUND_VERSION)
      printf "  %-30s %-15s  ⚠  version not in BD KB (component found)\n" "$name" "$version" >&2
      echo "$component"
      ;;
    *)
      if [ "$result" = "$orig_purl" ]; then
        printf "  %-30s %-15s  ✓  unchanged\n" "$name" "$version" >&2
      else
        printf "  %-30s %-15s  ✓  %s\n" "$name" "$version" "$result" >&2
      fi
      echo "$component" | jq --arg purl "$result" '.purl = $purl'
      ;;
  esac

  # Rate-limit: avoid hammering the BD API
  sleep 0.2
done | jq -s '.')

# ── Reconstruct the full CycloneDX document with updated components ──────────
UPDATED_DOC=$(jq \
  --argjson components "$UPDATED_COMPONENTS" \
  '.components = $components' \
  "$INPUT")

echo "" >&2
if $DRY_RUN; then
  echo "Dry run — showing updated document (not writing file):" >&2
  echo "$UPDATED_DOC"
else
  echo "$UPDATED_DOC" > "$INPUT"
  echo "Written to $INPUT" >&2
fi
