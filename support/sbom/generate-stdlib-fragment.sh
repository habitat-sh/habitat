#!/usr/bin/env bash
# Generate a CycloneDX 1.4 JSON fragment for third-party Rust sysroot crates.
#
# Discovery strategy (no hardcoded crate names):
#   1. List *.rlib files in the sysroot for the host target triple — these are
#      the crates actually compiled into the sysroot.
#   2. Filter to only crates present in the workspace Cargo.lock —
#      these are the ones actually used by this project. Versions come from
#      Cargo.lock (authoritative).
#   3. Emit CycloneDX 1.4 JSON to stdout.
#
# Usage:
#   bash support/sbom/generate-stdlib-fragment.sh > stdlib-sysroot.cdx.json
#
# Requires: rustc, rustup, jq

set -euo pipefail

SYSROOT=$(rustc --print sysroot)
HOST_TRIPLE=$(rustc --version --verbose | grep '^host:' | awk '{print $2}')
RLIB_DIR="$SYSROOT/lib/rustlib/$HOST_TRIPLE/lib"
CARGO_LOCK="${CARGO_LOCK:-Cargo.lock}"

echo "Sysroot:      $SYSROOT" >&2
echo "Host triple:  $HOST_TRIPLE" >&2
echo "Cargo.lock:   $CARGO_LOCK" >&2

if [ ! -f "$CARGO_LOCK" ]; then
  echo "ERROR: Cargo.lock not found at $CARGO_LOCK" >&2
  exit 1
fi

# --- Step 0: build name→version map from workspace Cargo.lock ---------------
# This is the authoritative version source for crates the project actually uses.
# Store both hyphen and underscore forms for flexible lookup.
declare -A CARGO_LOCK_VERSIONS
current_name=""
while IFS= read -r line; do
  if [[ "$line" =~ ^\[\[package\]\] ]]; then
    current_name=""
  elif [[ "$line" =~ ^name\ *=\ *\"(.+)\" ]]; then
    current_name="${BASH_REMATCH[1]}"
  elif [[ "$line" =~ ^version\ *=\ *\"(.+)\" ]] && [ -n "$current_name" ]; then
    ver="${BASH_REMATCH[1]}"
    CARGO_LOCK_VERSIONS["$current_name"]="$ver"
    underscore="${current_name//-/_}"
    hyphen="${current_name//_/-}"
    [ "$underscore" != "$current_name" ] && CARGO_LOCK_VERSIONS["$underscore"]="$ver"
    [ "$hyphen"     != "$current_name" ] && CARGO_LOCK_VERSIONS["$hyphen"]="$ver"
    current_name=""
  fi
done < "$CARGO_LOCK"
echo "Crates in Cargo.lock: ${#CARGO_LOCK_VERSIONS[@]} (name variants)" >&2

# --- Step 1: extract crate names from rlib filenames -----------------------
# rlib format: lib{crate_name}-{16+hex}.rlib
# Collect both underscore and hyphen forms to match Cargo.toml names.
RLIB_CRATES_ALL=$(
  find "$RLIB_DIR" -maxdepth 1 -name '*.rlib' \
    | sed -E 's|.*/lib||; s|-[0-9a-f]{10,}\.rlib$||' \
    | awk '{ h=$0; gsub(/_/,"-",h); print $0; if (h!=$0) print h }' \
    | sort -u
)
echo "Rlib entries (underscore+hyphen forms): $(echo "$RLIB_CRATES_ALL" | wc -l | tr -d ' ')" >&2

# First-party Rust crates to exclude even if they appear in rlibs
FIRST_PARTY="std|core|alloc|test|proc_macro|panic_abort|panic_unwind|unwind|rustc_std_workspace_core|rustc_std_workspace_alloc|rustc_std_workspace_std"

# --- Step 3: intersect rlib crates with Cargo.lock allowlist ----------------
# Include a sysroot crate only if it appears in the workspace Cargo.lock.
# SYSROOT_PAIRS is a bash array of "canonical-name version" strings.
# SEEN is an associative array used for O(1) deduplication (both underscore and
# hyphen rlib variants normalise to the same canonical hyphen form).
declare -a SYSROOT_PAIRS=()
declare -A SEEN=()
while IFS= read -r rlib_name; do
  # Skip first-party crates
  if echo "$rlib_name" | grep -qE "^(${FIRST_PARTY})$"; then
    continue
  fi
  # Only include crates referenced by this workspace
  if [ -z "${CARGO_LOCK_VERSIONS[$rlib_name]+_}" ]; then
    continue
  fi
  # Get version from Cargo.lock (authoritative; key is guaranteed present by the check above)
  ver="${CARGO_LOCK_VERSIONS[$rlib_name]}"
  canonical="${rlib_name//_/-}"
  # Deduplicate via associative array (both foo_bar and foo-bar map to foo-bar)
  if [ -z "${SEEN[$canonical]+_}" ]; then
    SEEN[$canonical]=1
    SYSROOT_PAIRS+=("$canonical $ver")
  fi
done <<< "$RLIB_CRATES_ALL"

if [ ${#SYSROOT_PAIRS[@]} -eq 0 ]; then
  echo "ERROR: No third-party sysroot crates found after intersection." >&2
  echo "  RLIB_DIR: $RLIB_DIR" >&2
  exit 1
fi

echo "Third-party sysroot crates:" >&2
printf '%s\n' "${SYSROOT_PAIRS[@]}" | sort | while IFS=' ' read -r n v; do echo "  $n $v" >&2; done

# --- Step 4: build CycloneDX 1.4 JSON with jq ------------------------------
COMPONENTS_JSON=$(
  printf '%s\n' "${SYSROOT_PAIRS[@]}" | sort \
  | while IFS=' ' read -r name ver; do
      jq -n \
        --arg name "$name" \
        --arg ver  "$ver"  \
        '{
          type:    "library",
          name:    $name,
          version: $ver,
          purl:    ("pkg:cargo/" + $name + "@" + $ver)
        }'
    done \
  | jq -s '.'
)

jq -n \
  --argjson components "$COMPONENTS_JSON" \
  '{
    bomFormat:   "CycloneDX",
    specVersion: "1.4",
    version:     1,
    metadata: {
      component: {
        type:        "library",
        name:        "rust-stdlib-sysroot-deps",
        description: "Third-party crates compiled into the Rust sysroot and statically linked into every Rust binary. Invisible to cargo metadata/cargo tree. Generated dynamically from the installed toolchain."
      }
    },
    components: $components
  }'

