#!/usr/bin/env bash

# # Internals
source_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
source "${source_dir}/public.bash"
source "${source_dir}/shared.bash"
source "${source_dir}/environment.bash"

# If this a native package build it does not require any sandboxing.
# So we directly execute the build
if [[ -n $HAB_NATIVE_PACKAGE ]]; then
  exec /usr/bin/env bash "${source_dir}/hab-plan-build-darwin-internal.bash" "$@"
fi

# Fail when commands return a non-zero return code.
set -e
set -E

# If the variable `$DEBUG` is set, then print all the
# shell commands as we execute.
if [[ -n "$DEBUG" ]]; then
  set -x
fi

# ## Default variables

# The short version of the program name which is used in logging output
_program=$(basename "$0")
# The root path of the Habitat file system. If the `$HAB_ROOT_PATH` environment
# variable is set, this value is overridden, otherwise it is set to its default
: "${HAB_ROOT_PATH:=/opt/hab}"
# The default path where source artifacts are downloaded, extracted, & compiled
HAB_CACHE_SRC_PATH=$HAB_ROOT_PATH/cache/src
# The default download root path for package artifacts, used on package
# installation
HAB_CACHE_ARTIFACT_PATH=$HAB_ROOT_PATH/cache/artifacts
# The default path where cryptographic keys are stored. If the
# `$HAB_CACHE_KEY_PATH` environment variable is set, this value is overridden,
# otherwise it is set to its default.
: "${HAB_CACHE_KEY_PATH:=$HAB_ROOT_PATH/cache/keys}"
# Export the key path for other programs and subshells to use
export HAB_CACHE_KEY_PATH
# The root path containing all locally installed packages
HAB_PKG_PATH=$HAB_ROOT_PATH/pkgs
# The first argument to the script is a Plan context directory, containing a
# `plan.sh` file
PLAN_CONTEXT=${1:-.}
ORIG_PLAN_CONTEXT=${PLAN_CONTEXT}
# The filename of the plan file
HAB_PLAN_FILENAME="plan.sh"
# The default Habitat Depot from where to download dependencies. If
# `HAB_BLDR_URL` is set, this value is overridden.
: "${HAB_BLDR_URL:=https://bldr.habitat.sh}"
# Export the Builder URL so all other programs and subshells use this same one
export HAB_BLDR_URL
# The default Habitat channel from where to download dependencies. If
# `HAB_BLDR_CHANNEL` is set, this value is overridden.
: "${HAB_BLDR_CHANNEL:=stable}"
# Export Builder channel so all other programs and subshells use this same one
export HAB_BLDR_CHANNEL
# Fall back here if package can't be installed from $HAB_BLDR_CHANNEL
# This is overridable with the sole intention of supporting core plans
# refresh evaluations (where we want to pull dependencies from a
# separate channel, and not "stable").
#
# Also note that this only really comes into play if HAB_BLDR_CHANNEL
# has been set to something different.
: "${HAB_FALLBACK_CHANNEL=stable}"
# The value of `$PATH` on initial start of this program
INITIAL_PATH="$PATH"
# The value of `pwd` on initial start of this program
INITIAL_PWD="$(pwd)"
# The compression level to use when compression harts (0..9)
: "${HAB_HART_COMPRESSION_LEVEL:=6}"
# The full target tuple this plan will be built for
pkg_target='aarch64-darwin'
# The package's origin (i.e. acme)
pkg_origin=""
# Each release is a timestamp - `YYYYMMDDhhmmss`
pkg_release=$(date -u +%Y%m%d%H%M%S)
# The default build deps setting - an empty array
pkg_build_deps=()
# The default runtime deps setting - an empty array
pkg_deps=()

# We want everything to be build as `rwx-r-x-r-x`
umask 0022

# In order to ensure that the external environment does not affect the build
# time behavior of a Plan, we explicitly unset several know environment
# variables which are known to be used by underlying autoconf-like tools
# and other build software.
unset TERMINFO

source "${source_dir}/internal.sh"

# Call the `_on_exit()` function above on:
# * HUP (1)
# * INT (2)
# * QUIT (3)
# * TERM (15)
# * ERR - when a shell command raises an error. Useful for `set -e; set -E`
#   above.
# * EXIT - ensure we wrap things up properly (including firing any
#   `do_after_failure` callback) if any thing calls `exit` (such as `exit_with`)
#
# See also `_do_final_callback_wrapper()`; it does some trap munging, too.
trap _on_exit 1 2 3 15 ERR EXIT

# # Main Flow
########################################################################
# Parse depot flag (-u)
OPTIND=2
while getopts "u:" opt; do
  case "${opt}" in
  u)
    HAB_BLDR_URL=$OPTARG
    ;;
  \?)
    echo "Invalid option: -$OPTARG" >&2
    exit 1
    ;;
  :)
    echo "Option -$OPTARG requires an argument." >&2
    exit 1
    ;;
  esac
done

# If the value of `$pkg_target` is a replacement token, then the program is
# being run out of a raw source tree (which must be supported), otherwise
# `$pkg_target` would have a static value set when the Habitat package for this
# program was built.
if [[ "$pkg_target" == "@@pkg_target@@" ]]; then
  if [[ -n "${BUILD_PKG_TARGET:-}" ]]; then
    # If a build environment variable is set with the desired package target,
    # then update the value of `$pkg_target`. This case is used in
    # bootstrapping the Habitat packaging system.
    pkg_target="$BUILD_PKG_TARGET"
    unset BUILD_PKG_TARGET
    build_line "Setting pkg_target='$pkg_target' from \$BUILD_PKG_TARGET"
  else
    # Otherwise, attempt to detect a suitable value for `$pkg_target` by using
    # the `uname` program. This is prior behavior and is backwards compatible
    # and behavior-preserving.
    _pkg_arch="$(uname -m | tr '[:upper:]' '[:lower:]')"
    _pkg_sys="$(uname -s | tr '[:upper:]' '[:lower:]')"
    pkg_target="${_pkg_arch}-${_pkg_sys}"
    unset _pkg_arch _pkg_sys
    build_line "Setting pkg_target='$pkg_target' using \`uname' detection"
  fi
fi

build_line "Plan Context is ----- $PLAN_CONTEXT"
# Expand the context path to an absolute path
PLAN_CONTEXT="$(abspath "$PLAN_CONTEXT")"
# Set the initial source root to be the same as the Plan context directory.
# This assumes that your application source is local and your Plan exists with
# your code.
SRC_PATH="${HAB_SRC_PATH:=$PLAN_CONTEXT}"

# Now to ensure a `plan.sh` exists where we expect it. There are 4 possible
# candidate locations relative to the `$PLAN_CONTEXT` directory:
#   `./plan.sh`
#   `./habitat/plan.sh`
#   `./$pkg_target/plan.sh`
#   `./habitat/$pkg_target/plan.sh`
# In most cases, Plan authors should use the default location of `./plan.sh`
# or `./habitat/plan.sh`.  The exception to this is when the $pkg_target
# requires variations to the default `plan.sh`. Plan authors can create these
# variants by placing a plan file in the appropriate $pkg_target directory
# relative to the default plan.sh.
#
# A plan found in the target folder will take precedence above a non-target
# folder. We currently allow a plan to exist both inside and outside of a
# target folder to support some core plans that have a Linux kernel 2 plan
# in a target folder and a Linux plan outside. Today we will warn in this
# condition but we should change those plans and then make this an error
# prompting a failure. If we find an invalid combination or are unable to
# find a plan.sh, abort with a message to the user with the failure case.
target_paths=()
paths=()
final_paths=()
candidate_target_paths=(
  "$PLAN_CONTEXT/$pkg_target/$HAB_PLAN_FILENAME"
  "$PLAN_CONTEXT/habitat/$pkg_target/$HAB_PLAN_FILENAME"
)
candidate_paths=(
  "$PLAN_CONTEXT/$HAB_PLAN_FILENAME"
  "$PLAN_CONTEXT/habitat/$HAB_PLAN_FILENAME"
)

# Lets notate all of the existing plan paths
for path in "${candidate_target_paths[@]}"; do
  if [[ -f $path ]]; then
    target_paths+=("$path")
  fi
done

for path in "${candidate_paths[@]}"; do
  if [[ -f $path ]]; then
    paths+=("$path")
  fi
done

if [[ ${#paths[@]} -gt 0 && ${#target_paths[@]} -gt 0 ]]; then
  warn "There is a $HAB_PLAN_FILENAME inside $pkg_target and outside as well. Using the plan in $pkg_target."
  warn "It is advisable to either remove the plan that is outside $pkg_target"
  warn "or move that plan to its own target folder if it is intended for a different target."
fi

# lets figure out what the final set of paths we are evaluating
# because target paths take precedence over non-target paths, we
# will use those if any were used
if [[ ${#target_paths[@]} -gt 0 ]]; then
  final_paths=("${target_paths[@]}")
else
  final_paths=("${paths[@]}")
fi

if [[ ${#final_paths[@]} -gt 1 ]]; then
  exit_with "A Plan file was found in the following paths: $(join_by ',' "${final_paths[@]}"). Only one is allowed at a time" 42
elif [[ ${#final_paths[@]} -eq 0 ]]; then
  all_paths=("${candidate_paths[@]}" "${candidate_target_paths[@]}")
  exit_with "Plan file not found in any of these paths: $(join_by ',' "${all_paths[@]}")" 42
else
  PLAN_CONTEXT="$(dirname "${final_paths[0]}")"
fi

# Load the Plan
build_line "Loading $PLAN_CONTEXT/$HAB_PLAN_FILENAME"
if source "$PLAN_CONTEXT/$HAB_PLAN_FILENAME"; then
  build_line "Plan loaded"
else
  ret=$?
  exit_with "Failed to load Plan" $ret
fi

if [[ -n "$HAB_ORIGIN" ]]; then
  pkg_origin="$HAB_ORIGIN"
fi

# Validate metadata
build_line "Validating plan metadata"

# Test for all required metadata keys
required_variables=(
  pkg_name
  pkg_origin
)
for var in "${required_variables[@]}"; do
  if [[ -z "${!var}" ]]; then
    exit_with "Failed to build. '${var}' must be set." 1
  fi
done

# Test to ensure package name contains only valid characters
for var in pkg_name pkg_origin; do
  if [[ ! "${!var}" =~ ^[A-Za-z0-9_-]+$ ]]; then
    exit_with "Failed to build. Package $var '${!var}' contains invalid characters." 1
  fi
done

# Ensure that the version is set (or can be set!) properly
if [[ -z "${pkg_version:-}" && "$(type -t pkg_version)" == "function" ]]; then
  pkg_version="__pkg__version__unset__"
elif [[ -z "${pkg_version:-}" ]]; then
  e="Failed to build. 'pkg_version' must be set or 'pkg_version()' function"
  e="$e must be implemented and then invoking by calling 'update_pkg_version()'."
  exit_with "$e" 1
fi

# If `$pkg_source` is used, default `$pkg_filename` to the basename of
# `$pkg_source` if it is not already set by the Plan.
if [[ -n "${pkg_source:-}" && -z "${pkg_filename+xxx}" ]]; then
  pkg_filename="$(basename "$pkg_source")"
fi

# Set `$pkg_dirname` to the `$pkg_name` and `$pkg_version`, if it is not
# already set by the Plan.
if [[ -z "${pkg_dirname+xxx}" ]]; then
  # shellcheck disable=2154
  pkg_dirname="${pkg_name}-${pkg_version}"
  _pkg_dirname_initially_unset=true
fi

if [[ -n "$HAB_OUTPUT_PATH" ]]; then
  pkg_output_path="$HAB_OUTPUT_PATH"
else
  pkg_output_path="$INITIAL_PWD/results"
fi

# Determine if we have all the commands we need to work
_find_system_commands
_determine_hab_bin

# We removed support for composite plans in habitat 0.75.0.  Let's give a useful error message
# if you happen to try and build one rather than create a useless package
if [[ -n "${pkg_type:-}" && "${pkg_type}" == "composite" ]]; then
  exit_with "Composite plans are no longer supported. For more details see https://www.habitat.sh/blog/2018/10/shelving-composites/" 1
fi

# Ensure that the origin key is available for package signing
_ensure_origin_key_present

_resolve_dependencies

SANDBOX_PROFILE="${HAB_STUDIO_ROOT}/tmp/PLAN_SANDBOX"
(
  echo ";; Generated sandbox profile"
  echo ""
) >"$SANDBOX_PROFILE"
cat "$source_dir/darwin-sandbox.sb" >>"$SANDBOX_PROFILE"

# `pkg_tdeps_resolved` set above in `_resolve_dependencies`
# shellcheck disable=2154
for dep in "${pkg_all_tdeps_resolved[@]}"; do
  runtime_sandbox_file="${dep}/RUNTIME_SANDBOX"
  if [[ -f "$runtime_sandbox_file" ]]; then
    (
      echo ""
      echo ";; Rules imported from $runtime_sandbox_file"
      echo "(import \"$runtime_sandbox_file\")"
    ) >>"$SANDBOX_PROFILE"
  fi
done

# This function writes out the runtime sandbox to a temporary file and validates
# if there are any syntax or other compilation errros in the sandbox.
# It does this by executing the /bin/false binary with the temporary sandbox profile and checking
# the error return code. If the return code is 65 it indicates a mistake in the sandbox profile.
# We do not care if the /bin/false execution fails or succeeds
if declare -f runtime_sandbox >/dev/null; then
  runtime_sandbox >"${HAB_STUDIO_ROOT}/tmp/RUNTIME_SANDBOX"
  ret=$(/usr/bin/sandbox-exec -f "${HAB_STUDIO_ROOT}/tmp/RUNTIME_SANDBOX" /bin/false 2>/dev/null || echo $?)
  if [[ "$ret" == "65" ]]; then
    out=$(/usr/bin/sandbox-exec -f "${HAB_STUDIO_ROOT}/tmp/RUNTIME_SANDBOX" /bin/false 2>&1 >/dev/null || echo "")
    exit_with "Invalid sandbox profile returned by 'runtime_sandbox' function:\\n$out"
  fi
fi

# This function writes out the buildtime sandbox to a temporary file and validates
# if there are any syntax or other compilation errros in the sandbox.
# It does this by executing the /bin/false binary with the temporary sandbox profile and checking
# the error return code. If the return code is 65 it indicates a mistake in the sandbox profile.
# We do not care if the /bin/false execution fails or succeeds
# It also adds a line at the end of the current sandbox profile to import the buildtime sandbox for this plan.
if declare -f buildtime_sandbox >/dev/null; then
  buildtime_sandbox >"${HAB_STUDIO_ROOT}/tmp/BUILDTIME_SANDBOX"
  ret=$(/usr/bin/sandbox-exec -f "${HAB_STUDIO_ROOT}/tmp/BUILDTIME_SANDBOX" /bin/false 2>/dev/null || echo $?)
  if [[ "$ret" == "65" ]]; then
    out=$(/usr/bin/sandbox-exec -f "${HAB_STUDIO_ROOT}/tmp/BUILDTIME_SANDBOX" /bin/false 2>&1 >/dev/null || echo "")
    exit_with "Invalid sandbox profile returned by 'buildtime_sandbox' function:\\n$out"
  fi
  (
    echo ""
    echo ";; Rules imported from plan buildtime_sandbox function"
    echo "(import \"${HAB_STUDIO_ROOT}/tmp/BUILDTIME_SANDBOX\")"
  ) >>"$SANDBOX_PROFILE"
fi

build_line "Sandbox profile written to $SANDBOX_PROFILE"
build_line "Plan Context is $PLAN_CONTEXT. SRC_PATH is $SRC_PATH"

exec /usr/bin/sandbox-exec \
  -f "$SANDBOX_PROFILE" \
  -DSTUDIO_DIR="$HAB_STUDIO_ROOT" \
  -DSTUDIO_HAB="$HAB_STUDIO_HAB_BIN" \
  -DPLAN_CONTEXT_DIR="$PWD" \
  -DPKG_OUTPUT_PATH="$pkg_output_path" \
  "${source_dir}/hab-plan-build-darwin-internal.bash" "${ORIG_PLAN_CONTEXT}" "${@:2}"
