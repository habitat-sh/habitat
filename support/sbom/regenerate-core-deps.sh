#!/usr/bin/env bash
# regenerate-core-deps.sh
#
# Regenerates support/sbom/habitat-core-deps.cyclonedx.json by querying the
# Builder API for the full transitive dependency trees of a set of seed
# packages across all supported platform targets.
#
# Strategy:
#   1. For each seed package × platform target, fetch the package metadata
#      from the Builder API and extract the tdeps list.
#   2. Filter to deps in the "core" origin only.
#   3. Additionally, fetch a set of "base channel extras" — core-origin
#      packages queried directly from BASE_CHANNEL — and include both the
#      package itself and its core-origin tdeps (each on its specified target).
#   4. Deduplicate by {name, version} pair. The same package may legitimately
#      appear at different versions across platforms; all distinct
#      {name, version} combinations are included as separate entries.
#   5. Merge with the existing JSON:
#        * Unchanged  – {name, version} pair already present → kept as-is
#                       (existing purl is preserved).
#        * New        – {name, version} pair not previously in the file →
#                       added with a placeholder purl:
#                       pkg:generic/{name}@{version}
#        * Removed    – {name, version} pair no longer in any dep tree → dropped.
#   6. Write the result back to the file, or print to stdout with --dry-run.
#
# Usage:
#   bash support/sbom/regenerate-core-deps.sh [--dry-run] [pkg ...]
#
#   Positional arguments (e.g. chef/hab chef/hab-sup) override the default
#   seed package list. Omit them to use the built-in defaults.
#
# Environment:
#   BLDR_URL      Builder base URL        (default: https://bldr.habitat.sh)
#   CHANNEL       Channel for seed pkgs   (default: acceptance)
#   BASE_CHANNEL  Channel for base extras (default: base)
#
# Requires: curl, jq

set -euo pipefail

BLDR_URL="${BLDR_URL:-https://bldr.habitat.sh}"
CHANNEL="${CHANNEL:-acceptance}"
BASE_CHANNEL="${BASE_CHANNEL:-base}"
OUTPUT="support/sbom/habitat-core-deps.cyclonedx.json"
DRY_RUN=false

# --------------------------------------------------------------------------
# Parse arguments
# --------------------------------------------------------------------------
POSITIONAL_ARGS=()
for arg in "$@"; do
  case "$arg" in
    --dry-run) DRY_RUN=true ;;
    --help|-h)
      sed -n '2,/^set -euo/{ /^set -euo/d; s/^# \{0,1\}//; p }' "$0"
      exit 0
      ;;
    *) POSITIONAL_ARGS+=("$arg") ;;
  esac
done

# Seed packages whose full transitive dep trees define this SBOM fragment.
# Override by passing package idents as positional arguments.
DEFAULT_SEEDS=(
  "chef/hab"
  "chef/hab-sup"
  "chef/hab-launcher"
  "chef/hab-studio"
  "chef/hab-pkg-export-tar"
  "chef/hab-pkg-export-container"
  "chef/hab-plan-build-ps1"
  "chef/windows-service"
)
if [ "${#POSITIONAL_ARGS[@]}" -gt 0 ]; then
  SEED_PACKAGES=("${POSITIONAL_ARGS[@]}")
else
  SEED_PACKAGES=("${DEFAULT_SEEDS[@]}")
fi

# Platform targets supported by Habitat (x86_64-darwin excluded).
TARGETS=(
  "x86_64-linux"
  "aarch64-linux"
  "x86_64-windows"
  "aarch64-darwin"
)

echo "Builder URL:   $BLDR_URL"   >&2
echo "Channel:       $CHANNEL"    >&2
echo "Base channel:  $BASE_CHANNEL" >&2
echo "Targets:       ${TARGETS[*]}" >&2
echo "Seed packages: ${SEED_PACKAGES[*]}" >&2
echo "" >&2

# --------------------------------------------------------------------------
# Step 1: Collect all core-origin transitive deps from the Builder API
# --------------------------------------------------------------------------
# CORE_DEPS is a set keyed by "name@version".
# CORE_DEP_META["name@version"] = "name version" for easy extraction later.
declare -A CORE_DEPS       # key: "name@version" → value: "1"
declare -A CORE_DEP_META   # key: "name@version" → value: "name version"

add_dep() {
  local name="$1" version="$2"
  local key="${name}@${version}"
  if [ -z "${CORE_DEPS[$key]+_}" ]; then
    CORE_DEPS["$key"]="1"
    CORE_DEP_META["$key"]="${name} ${version}"
  fi
}

for pkg in "${SEED_PACKAGES[@]}"; do
  origin="${pkg%%/*}"
  pkg_name="${pkg##*/}"
  for target in "${TARGETS[@]}"; do
    url="${BLDR_URL}/v1/depot/channels/${origin}/${CHANNEL}/pkgs/${pkg_name}/latest?target=${target}"
    printf "  Fetching %-40s [%-16s] ... " "${pkg}" "${target}" >&2
    if response=$(curl -sSf "$url" 2>/dev/null); then
      mapfile -t deps < <(
        echo "$response" \
          | jq -r '.tdeps[]? | select(.origin == "core") | "\(.name)/\(.version)"' \
          2>/dev/null \
        || true
      )
      printf "%d core deps\n" "${#deps[@]}" >&2
      for dep in "${deps[@]}"; do
        [ -z "$dep" ] && continue
        add_dep "${dep%%/*}" "${dep##*/}"
      done
    else
      printf "not found (skipping)\n" >&2
    fi
  done
done

# --------------------------------------------------------------------------
# Base channel extras: core-origin packages fetched from BASE_CHANNEL on a
# specific target. Both the package itself and its core-origin tdeps are added.
# Format: "origin/name:target"
# --------------------------------------------------------------------------
BASE_EXTRAS=(
  "core/7zip:x86_64-windows"
)

echo "" >&2
echo "Fetching base channel extras (channel: $BASE_CHANNEL) ..." >&2
for entry in "${BASE_EXTRAS[@]}"; do
  pkg="${entry%%:*}"
  target="${entry##*:}"
  origin="${pkg%%/*}"
  pkg_name="${pkg##*/}"
  url="${BLDR_URL}/v1/depot/channels/${origin}/${BASE_CHANNEL}/pkgs/${pkg_name}/latest?target=${target}"
  printf "  Fetching %-40s [%-16s] ... " "${pkg}" "${target}" >&2
  if response=$(curl -sSf "$url" 2>/dev/null); then
    # Include the package itself.
    self_version=$(echo "$response" | jq -r '.ident.version' 2>/dev/null || true)
    if [ -n "$self_version" ]; then
      add_dep "$pkg_name" "$self_version"
    fi
    # Include its core-origin tdeps.
    mapfile -t deps < <(
      echo "$response" \
        | jq -r '.tdeps[]? | select(.origin == "core") | "\(.name)/\(.version)"' \
        2>/dev/null \
      || true
    )
    printf "%d core deps (+ self)\n" "${#deps[@]}" >&2
    for dep in "${deps[@]}"; do
      [ -z "$dep" ] && continue
      add_dep "${dep%%/*}" "${dep##*/}"
    done
  else
    printf "not found (skipping)\n" >&2
  fi
done

echo "" >&2
echo "Unique core {name, version} pairs found: ${#CORE_DEPS[@]}" >&2
echo "" >&2

if [ "${#CORE_DEPS[@]}" -eq 0 ]; then
  echo "ERROR: No core-origin packages found. Check BLDR_URL, CHANNEL, and seed packages." >&2
  exit 1
fi

# --------------------------------------------------------------------------
# Step 2: Merge resolved deps with the existing JSON
# --------------------------------------------------------------------------
echo "Merging with existing ${OUTPUT} ..." >&2
echo "" >&2

component_jsons=()

# Sort keys as "name@version" so output is deterministic (alphabetically by
# name, then by version for any same-name entries).
for key in $(printf '%s\n' "${!CORE_DEP_META[@]}" | sort); do
  read -r pkg_name version <<< "${CORE_DEP_META[$key]}"
  display_name="Habitat core_${pkg_name}"

  # Look up existing entry matching both name and version.
  existing_purl=$(
    jq -r \
      --arg n "$display_name" \
      --arg v "$version" \
      '(.components[] | select(.name == $n and .version == $v) | .purl) // ""' \
      "$OUTPUT" 2>/dev/null \
    || true
  )

  if [ -n "$existing_purl" ]; then
    purl="$existing_purl"
    printf "  [unchanged]  %s  %s\n" "$pkg_name" "$version" >&2
  else
    purl="pkg:generic/${pkg_name}@${version}"
    printf "  [new]        %s  %s\n" "$pkg_name" "$version" >&2
  fi

  component_jsons+=(
    "$(jq -cn \
      --arg name    "$display_name" \
      --arg version "$version" \
      --arg purl    "$purl" \
      '{type: "library", name: $name, version: $version, purl: $purl}')"
  )
done

# Report packages that will be removed (present in existing file but not in new tree).
echo "" >&2
while IFS= read -r entry; do
  existing_name=$(echo "$entry" | jq -r '.name')
  existing_version=$(echo "$entry" | jq -r '.version')
  pkg_suffix="${existing_name#Habitat core_}"
  key="${pkg_suffix}@${existing_version}"
  if [ -z "${CORE_DEPS[$key]+_}" ]; then
    printf "  [removed]    %s  %s\n" "$pkg_suffix" "$existing_version" >&2
  fi
done < <(jq -c '.components[]' "$OUTPUT")

# --------------------------------------------------------------------------
# Step 3: Build and write the final CycloneDX document
# --------------------------------------------------------------------------
components_json=$(printf '%s\n' "${component_jsons[@]}" | jq -s '.')

updated_doc=$(
  jq --argjson components "$components_json" \
    '.components = $components' \
    "$OUTPUT"
)

echo "" >&2
if $DRY_RUN; then
  echo "Dry run — showing updated document (not writing to file):" >&2
  echo "$updated_doc"
else
  echo "$updated_doc" > "$OUTPUT"
  echo "Written to $OUTPUT" >&2
fi
