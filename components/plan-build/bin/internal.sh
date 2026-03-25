#!/usr/bin/env bash
# shellcheck disable=2154

_artifact_ext="hart"

# **Internal** Prints out an optionally colorized message indicating
# an error of some kind.
#
# The line will be indented, with the package name in blue, and
# `message` in red.
_build_error_message() {
  local message=${1}
  : ${pkg_name:=unknown}

  if [[ "${HAB_NOCOLORING:-}" == "true" ]]; then
    echo "   ${pkg_name}: ${message}"
  else
    case "${TERM:-}" in
    *term | xterm-* | rxvt | screen | screen-*)
      echo -e "   \033[1;36m${pkg_name}: \033[1;31m${message}\033[0m"
      ;;
    *)
      echo "   ${pkg_name}: ${message}"
      ;;
    esac
  fi
}

# ## Private/Internal helper functions
#
#
# These functions are part of the private/internal API of this program and
# should **not** be used externally by Plan authors or other programs. Their
# behaviors and names can change with little to no warning and no direct
# support can be provided as a result. Thank you for your
# understanding--maintaining a tiny but robust public interface is not an easy
# task.

# **Internal** Handles exiting the program on signals. Takes either an argument
# with the status code, or uses the last command's status code.
#
# For example, the below would exit with a status of 1.
#
# ```bash
# _on_exit 1
# ```
#
# Or:
#
# ```bash
# false
# _on_exit
# ```
#
# Would also exit 1.
_on_exit() {
  # TODO (CM): would be great to show where in the plan the build failed

  local exit_status=${1:-$?}
  if [[ $BASH_SUBSHELL -gt 0 ]]; then
    exit "$exit_status"
  fi
  : "${pkg_name:=unknown}"
  elapsed=$SECONDS
  elapsed=$(echo $elapsed | awk '{printf "%dm%ds", $1/60, $1%60}')
  if [[ "${HAB_NOCOLORING:-}" == "true" ]]; then
    echo "   ${pkg_name}: Sandbox profile resolution time: $elapsed"
  else
    case "${TERM:-}" in
    *term | xterm-* | rxvt | screen | screen-*)
      echo -e "   \033[1;36m${pkg_name}: \033[1;37mSandbox profile resolution: $elapsed\033[0m"
      ;;
    *)
      echo "   ${pkg_name}: Sandbox profile resolution: $elapsed"
      ;;
    esac
  fi
  if [[ $exit_status -ne 0 ]]; then
    _build_error_message "Exiting on error"
  fi

  # If we don't unset the trap for EXIT, we can end up double-exiting,
  # which could cause a `do_after_failure` callback to fire twice.
  trap - EXIT
  exit "$exit_status"
}

_ensure_origin_key_present() {
  local cache="$HAB_CACHE_KEY_PATH"
  local keys_found
  keys_found="$(find "$cache" -name "${pkg_origin}-*.sig.key" | wc -l)"
  if [[ $keys_found -eq 0 ]]; then
    exit_with "Signing origin key '$pkg_origin' not found in $cache, aborting" 35
  fi
  debug "At least one signing key for $pkg_origin found in $cache"
}

# **Internal** Ensures that the correct versions of key system commands are
# able to be used by this program. If we cannot find suitable versions, we will
# abort early.
#
# The following variables are set which contain an absolute path to the desired
# command:
#
# * `$_hab_cmd` (hab cli for signing, hashing, and possibly installing)
# * `$_stat_cmd` (either GNU or BSD stat on system)
# * `$_wget_cmd` (wget on system)
# * `$_shasum_cmd` (either gsha256sum or sha256sum on system)
# * `$_tar_cmd` (GNU version of tar)
# * `$_mktemp_cmd` (GNU version from coreutils)
# * `$_toml_cmd`
#
# Note that all of the commands noted above are considered internal
# implementation details and are subject to change with little to no notice,
# which means the variables such as `$_tar_cmd` should **not** be used directly
# by Plan authors. The leading underscore denotes an internal/private variable
# or function.

# If the commands are not found, `exit_with` is called and the program is
# terminated.
_find_system_commands() {
  if exists stat; then
    _stat_cmd=$(command -v stat)
    if $_stat_cmd -f '%Su:%g' . 2>/dev/null 1>/dev/null; then
      _stat_variant="bsd"
    elif $_stat_cmd -c '%u:%g' . 2>/dev/null 1>/dev/null; then
      _stat_variant="gnu"
    else
      exit_with "Failed to determine stat variant, we require GNU or BSD stat to determine user and group owners of files; aborting" 1
    fi
  else
    exit_with "We require GNU or BSD stat to determine user and group owners of files; aborting" 1
  fi
  if exists wget; then
    _wget_cmd=$(command -v wget)
    if [[ "${HAB_NONINTERACTIVE:-}" == "true" ]]; then
      _wget_cmd="$_wget_cmd --no-verbose"
    fi
  else
    exit_with "We require wget to download sources; aborting" 1
  fi
  debug "Setting _wget_cmd=$_wget_cmd"

  if exists gsha256sum; then
    _shasum_cmd=$(command -v gsha256sum)
  elif exists sha256sum; then
    _shasum_cmd=$(command -v sha256sum)
  else
    exit_with "We require gsha256sum or sha256sum for verifying the downloaded source; aborting" 1
  fi
  debug "Setting _shasum_cmd=$_shasum_cmd"

  if tar --version 2>&1 | grep -q 'GNU tar'; then
    _tar_cmd=$(command -v tar)
  else
    if /bin/tar --version 2>&1 | grep -q 'GNU tar'; then
      _tar_cmd=/bin/tar
    else
      exit_with "We require GNU tar for long path support; aborting" 1
    fi
  fi
  debug "Setting _tar_cmd=$_tar_cmd"

  if exists xz; then
    _xz_cmd=$(command -v xz)
    if [[ "${HAB_NONINTERACTIVE:-}" != "true" ]]; then
      _xz_cmd="$_xz_cmd --verbose"
    fi
  else
    exit_with "We require xz to compress artifacts; aborting" 1
  fi
  debug "Setting _xz_cmd=$_xz_cmd"

  if exists hab; then
    _hab_cmd=$(command -v hab)
  else
    exit_with "We require hab to sign artifacts; aborting" 1
  fi
  debug "Setting _hab_cmd=$_hab_cmd"
}

# **Internal** Return the path to the latest release of a package on stdout.
#
# ```
# _latest_installed_package acme/nginx
# # /hab/pkgs/acme/nginx/1.8.0/20150911120000
# _latest_installed_package acme/nginx/1.8.0
# # /hab/pkgs/acme/nginx/1.8.0/20150911120000
# _latest_installed_package acme/nginx/1.8.0/20150911120000
# # /hab/pkgs/acme/nginx/1.8.0/20150911120000
# ```
#
# Will return 0 if a package was found on disk, and 1 if a package cannot be
# found. A message will be printed to stderr explaining that no package was
# found.
_latest_installed_package() {
  local result
  if result="$($HAB_BIN pkg path "$1" 2>/dev/null)"; then
    echo "$result"
    return 0
  else
    warn "Could not find a suitable installed package for '$1'"
    return 1
  fi
}

# **Internal** Returns the path to the desired package on stdout, using the
# constraints specified in `$pkg_deps` or `$pkg_build_deps`. If a package
# cannot be found locally on disk, and the `hab` CLI package is present,
# this program will attempt to install the package from a remote repository.
#
# ```
# _resolve_dependency acme/zlib
# # /hab/pkgs/acme/zlib/1.2.8/20151216221001
# _resolve_dependency acme/zlib/1.2.8
# # /hab/pkgs/acme/zlib/1.2.8/20151216221001
# _resolve_dependency acme/zlib/1.2.8/20151216221001
# # /hab/pkgs/acme/zlib/1.2.8/20151216221001
# ```
#
# Will return 0 if a package was found or installed on disk, and 1 if a package
# cannot be found or remotely installed. A message will be printed to stderr to
# provide context.
_resolve_dependency() {
  local dep="$1"
  local dep_path
  if ! echo "$dep" | grep -q '\/' >/dev/null; then
    warn "Origin required for '$dep' in plan '$pkg_origin/$pkg_name' (example: acme/$dep)"
    return 1
  fi

  if dep_path=$(_latest_installed_package "$dep"); then
    echo "${dep_path}"
    return 0
  else
    return 1
  fi
}

# **Internal** Attempts to download a package dependency. If the value of the
# `$NO_INSTALL_DEPS` variable is set, then no package installation will occur.
# If an installation is attempted but there is an error, this function will
# still return with `0` and is intended to be "best effort".
#
# ```
# _install_dependency acme/zlib
# _install_dependency acme/zlib/1.2.8
# _install_dependency acme/zlib/1.2.8/20151216221001
# ```
_install_dependency() {
  local dep="${1}"
  if [[ -z "${NO_INSTALL_DEPS:-}" ]]; then

    # Enable --ignore-local if invoked with HAB_FEAT_IGNORE_LOCAL in
    # the environment, set to either "true" or "TRUE" (features are
    # not currently enabled by the mere presence of an environment variable)
    if [[ "${HAB_FEAT_IGNORE_LOCAL:-}" = "true" ||
      "${HAB_FEAT_IGNORE_LOCAL:-}" = "TRUE" ]]; then
      IGNORE_LOCAL="--ignore-local"
    fi
    $HAB_BIN pkg install -u "$HAB_BLDR_URL" --channel "$HAB_BLDR_CHANNEL" ${IGNORE_LOCAL:-} "$@" || {
      if [[ "$HAB_BLDR_CHANNEL" != "$HAB_FALLBACK_CHANNEL" ]]; then
        build_line "Trying to install '$dep' from '$HAB_FALLBACK_CHANNEL'"
        $HAB_BIN pkg install -u "$HAB_BLDR_URL" --channel "$HAB_FALLBACK_CHANNEL" ${IGNORE_LOCAL:-} "$@" || true
      fi
    }
  fi
  return 0
}

# **Internal** Returns (on stdout) the `TDEPS` file contents of another locally
# installed package which contain the set of all direct and transitive run
# dependencies. An empty set generates no output. The lack of a `TDEPS` file or
# a TDEPS file of zero bytes in the desired package will be considered an unset,
# or empty set.
#
# ```
# _get_tdeps_for /hab/pkgs/acme/a/4.2.2/20160113044458
# # /hab/pkgs/acme/dep-b/1.2.3/20160113033619
# # /hab/pkgs/acme/dep-c/5.0.1/20160113033507
# # /hab/pkgs/acme/dep-d/2.0.0/20160113033539
# # /hab/pkgs/acme/dep-e/10.0.1/20160113033453
# # /hab/pkgs/acme/dep-f/4.2.2/20160113033338
# # /hab/pkgs/acme/dep-g/4.2.2/20160113033319
# ```
#
# Will return 0 in any case and the contents of `TDEPS` if the file exists.
#
# Syntax note:
#
# This function outputs package identifiers separated by newlines. To read this
# output into an array where each element is a package identifier, the proper
# syntax is
# ```
# mapfile -t array_of_pkg_idents < <(_get_tdeps_for "$pkg_path")
# ```
# The <(...) syntax is process substitution, which is necessary for mapfile
# to operate on the output this function generates as though it were a file.
# See https://www.gnu.org/software/bash/manual/html_node/Process-Substitution.html.
# The < beforehand is regular input redirection.
#
# See also https://github.com/koalaman/shellcheck/wiki/SC2207#prefer-mapfile-or-read--a-to-split-command-output-or-quote-to-avoid-splitting
# for why `mapfile` is necessary and why
# ```
# array_of_pkg_idents=($(_get_tdeps_for "$pkg_path"))
# ```
# Should be avoided.
_get_tdeps_for() {
  local pkg_path="${1?_get_tdeps_for requires a pkg_path argument}"
  if [[ -s "$pkg_path/TDEPS" ]]; then
    cat "$pkg_path"/TDEPS
  fi
}

# **Internal** Returns (on stdout) the `DEPS` file contents of another locally
# installed package which contain the set of all direct run dependencies. An
# empty set could be returned as whitespace and/or newlines. An empty set
# generates no output. The lack of a `DEPS` file or a DEPS file of zero bytes in
# the desired package will be considered an unset, or empty set.
#
# ```
# _get_deps_for /hab/pkgs/acme/a/4.2.2/20160113044458
# # /hab/pkgs/acme/dep-b/1.2.3/20160113033619
# # /hab/pkgs/acme/dep-c/5.0.1/20160113033507
# # /hab/pkgs/acme/dep-d/2.0.0/20160113033539
# # /hab/pkgs/acme/dep-e/10.0.1/20160113033453
# # /hab/pkgs/acme/dep-f/4.2.2/20160113033338
# # /hab/pkgs/acme/dep-g/4.2.2/20160113033319
# ```
#
# Will return 0 in any case and the contents of `DEPS` if the file exists.
#
# Syntax note:
# This function outputs package identifiers separated by newlines. To read this
# output into an array where each element is a package identifier, the proper
# syntax is
# ```
# mapfile -t array_of_pkg_idents < <(_get_deps_for "$pkg_path")
# ```
# The <(...) syntax is process substitution, which is necessary for mapfile
# to operate on the output this function generates as though it were a file.
# See https://www.gnu.org/software/bash/manual/html_node/Process-Substitution.html.
# The < beforehand is regular input redirection.
#
# See also https://github.com/koalaman/shellcheck/wiki/SC2207#prefer-mapfile-or-read--a-to-split-command-output-or-quote-to-avoid-splitting
# for why `mapfile` is necessary and why
# ```
# array_of_pkg_idents=($(_get_deps_for "$pkg_path"))
# ```
# Should be avoided.
_get_deps_for() {
  local pkg_path="${1?_get_deps_for requires a pkg_path argument}"
  if [[ -s "$pkg_path/DEPS" ]]; then
    cat "$pkg_path"/DEPS
  fi
}

# **Internal** Appends an entry to the given array only if the entry is not
# already present and returns the resulting array back on stdout. In so doing,
# this function mimics a set when adding new entries. Note that any array can
# be passed in, including ones that already contain duplicate entries.
#
# ```
# arr=(a b c)
# read -r -a arr <<< "$(_return_or_append_to_set "b" "${arr[@]}")"
# echo ${arr[@]}
# # a b c
# read -r -a arr <<< "$(_return_or_append_to_set "z" "${arr[@]}")"
# echo ${arr[@]}
# # a b c z
# ```
#
# Will return 0 in any case.
#
# Syntax note:
#
# This function outputs array elements separated by whitespace. To read this
# output into an updated array where the new element is added only if it did
# not previous exist in the array, the syntax is
# ```
# read -r -a output_array <<< "$(_return_or_append_to_set "$new_element" "${input_array[@]}")"
# ```
# The <<< syntax is a here string, which is necessary to treat the output of
# the function as a single line of input on stdin from the perspective of
# the `read` built-in.
# See https://www.gnu.org/software/bash/manual/html_node/Redirections.html#Here-Strings
#
# See also https://github.com/koalaman/shellcheck/wiki/SC2207#prefer-mapfile-or-read--a-to-split-command-output-or-quote-to-avoid-splitting
# for why `read` is necessary and why
# ```
# output_array=($(_return_or_append_to_set "new_element" "${input_array[@]}"))
# ```
# Should be avoided.
_return_or_append_to_set() {
  local appended_set
  if _array_contains "$1" "${@:2}"; then
    echo "${@:2}"
    return 0
  fi
  appended_set=("${@:2}" "$1")
  echo "${appended_set[@]}"
  return 0
}

# **Internal** Appends an entry to the given array and removes any entries
# already in the array that match the passed entry. Note that this is specifically
# intended for lists of dependencies used for building TDEPS metadata. This should
# not be used as a generic array appender because it could have unexpected results
# with arbitrary data.
_add_dep_to_tdep_list() {
  local to_add="${1}"
  local deps=("${@:2}")
  local result=()

  # Explicitly filter out any instances of the to-be-added dependency
  # that we may have already seen.
  for d in "${deps[@]}"; do
    if [[ "$d" != "${to_add}" ]]; then
      result=("${result[@]}" "$d")
    fi
  done
  # Append the dependency to the end of the list
  result=("${result[@]}" "$to_add")

  echo "${result[@]}"
  return 0
}

# **Internal** Returns 0 (true) if the element is present in the array and
# non-zero (false) otherwise.
#
# ```
# arr=(a b c)
# [[ $(_array_contains "b" "${arr[@]}") -eq 0 ]]
#
# [[ $(_array_contains "nope" "${arr[@]}") -ne 0 ]]
# ```
_array_contains() {
  local e
  for e in "${@:2}"; do
    if [[ "$e" == "$1" ]]; then
      return 0
    fi
  done
  return 1
}

# **Internal** Determines what command/binary to use for installation of
# package dependencies, signing, and hashing files. The `$HAB_BIN` variable
# will either be set or emptied according to the following criteria (first
# match wins):
#
# * If a `$HAB_BIN` environment variable is set, then use this as the absolute
#   path to the binary.
# * Otherwise `$_hab_cmd` is used, set in the `_find_system_commands()`
#   function
_determine_hab_bin() {
  if [[ -n "${NO_INSTALL_DEPS:-}" ]]; then
    build_line "NO_INSTALL_DEPS set: no package dependencies will be installed"
  fi

  if [[ -z "${HAB_BIN:-}" ]]; then
    HAB_BIN="$_hab_cmd"
  fi
  build_line "Using HAB_BIN=$HAB_BIN for installs, signing, and hashing"
}

# TODO (CM): why can't these arrays be created up at the top of this
# script? What does having a separate function get for us? Should we
# pull these up to the top, or put some of the things up top into
# their own functions?

# **Internal** Create initial package-related arrays.
_init_dependencies() {
  # Create `${pkg_build_deps_resolved[@]}` containing all resolved direct build
  # dependencies.
  pkg_build_deps_resolved=()

  # Create `${pkg_build_tdeps_resolved[@]}` containing all the direct build
  # dependencies, and the run dependencies for each direct build dependency.
  pkg_build_tdeps_resolved=()

  # Create `${pkg_deps_resolved[@]}` containing all resolved direct run
  # dependencies.
  pkg_deps_resolved=()

  # Create `${pkg_tdeps_resolved[@]}` containing all the direct run
  # dependencies, and the run dependencies for each direct run dependency.
  pkg_tdeps_resolved=()
}

# **Internal** Installs the scaffolding dependencies and for each scaffolding
# package, add itself and each direct run dependency to the start of
# `${pkg_build_deps[@]}`. In this way, it would be as if the Plan author had
# added each of these dependencies directly into their `${pkg_build_deps[@]}`.
# Each of these direct run dependencies are fully qualified so that when
# resolving all build dependencies, only each specific package is locked down.
_resolve_scaffolding_dependencies() {
  if [[ -z "${pkg_scaffolding:-}" ]]; then
    return 0
  fi

  build_line "Resolving scaffolding dependencies"
  local resolved
  local dep
  local tdep
  local tdeps
  local sdep
  local sdeps
  local scaff_build_deps
  local scaff_build_deps_resolved

  scaff_build_deps=()
  scaff_build_deps_resolved=()

  _install_dependency "$pkg_scaffolding"
  # Add scaffolding package to the list of scaffolding build deps
  scaff_build_deps+=("$pkg_scaffolding")
  if resolved="$(_resolve_dependency "$pkg_scaffolding")"; then
    build_line "Resolved scaffolding dependency '$pkg_scaffolding' to $resolved"
    scaff_build_deps_resolved+=("$resolved")
    # Add each (fully qualified) direct run dependency of the scaffolding
    # package.
    mapfile -t sdeps < <(_get_deps_for "$resolved") # See syntax note @ _get_deps_for
    for sdep in "${sdeps[@]}"; do
      scaff_build_deps+=("$sdep")
      scaff_build_deps_resolved+=("$HAB_PKG_PATH/$sdep")
    done
  else
    exit_with "Resolving '$pkg_scaffolding' failed, should this be built first?" 1
  fi

  # Add all of the ordered scaffolding dependencies to the start of
  # `${pkg_build_deps[@]}` to make sure they could be overridden by a Plan
  # author if required.
  pkg_build_deps=("${scaff_build_deps[@]}" "${pkg_build_deps[@]}")
  debug "Updating pkg_build_deps=(${pkg_build_deps[*]}) from Scaffolding deps"

  # Set `pkg_build_deps_resolved[@]}` to all resolved scaffolding dependencies.
  # This will be used for early scaffolding package loading to mimic the state
  # where all dependencies are known for helpers such as `pkg_path_for` and
  # will be re-set later when the full build dependency set is known.
  pkg_build_deps_resolved=("${scaff_build_deps_resolved[@]}")
  # Set `${pkg_build_tdeps_resolved[@]}` to all the direct scaffolding
  # dependencies, and the run dependencies for each direct scaffolding
  # dependency. As above, this will be re-set later when the full dependency
  # set is known.
  _set_build_tdeps_resolved
}

# **Internal** Determines suitable package identifiers for each build
# dependency and populates several package-related arrays for use throughout
# this program.
#
# Walk each item in `$pkg_build_deps`, and for each item determine the absolute
# path to a suitable package release (which will be on disk).
_resolve_build_dependencies() {
  build_line "Resolving build dependencies"
  local resolved
  local dep
  local tdep
  local tdeps

  # Clear out any pre-existing entries for `${pkg_build_deps_resolved[@]}`
  # which may have been set by scaffolding preparation. All build dependencies
  # will now be processed together including any scaffolding-injected
  # dependencies.
  pkg_build_deps_resolved=()

  # Append to `${pkg_build_deps_resolved[@]}` all resolved direct build
  # dependencies.
  for dep in "${pkg_build_deps[@]}"; do
    _install_dependency "$dep"
    if resolved="$(_resolve_dependency "$dep")"; then
      build_line "Resolved build dependency '$dep' to $resolved"
      pkg_build_deps_resolved+=("$resolved")
    else
      exit_with "Resolving '$dep' failed, should this be built first?" 1
    fi
  done

  # Set `${pkg_build_tdeps_resolved[@]}` to all the direct build dependencies,
  # and the run dependencies for each direct build dependency.
  _set_build_tdeps_resolved
}

# **Internal** Sets the value of `${pkg_build_tdeps_resolved[@]}`. This
# function completely re-sets the value of `${pkg_build_tdeps_resolved[@]}`
# using the current value of `${pkg_build_deps_resolved[@]}`.
_set_build_tdeps_resolved() {
  # Copy all direct build dependencies into a new array
  pkg_build_tdeps_resolved=("${pkg_build_deps_resolved[@]}")
  # Append all non-direct (transitive) run dependencies for each direct build
  # dependency. That's right, not a typo ;) This is how a `acme/gcc` build
  # dependency could pull in `acme/binutils` for us, as an example. Any
  # duplicate entries are dropped to produce a proper set.
  for dep in "${pkg_build_deps_resolved[@]}"; do
    mapfile -t tdeps < <(_get_tdeps_for "$dep") # See syntax note @ _get_tdeps_for
    for tdep in "${tdeps[@]}"; do
      tdep="$HAB_PKG_PATH/$tdep"
      # Use _add_dep_to_tdep_list instead of _return_or_append_to_set
      # so that duplicate entries are removed from the top of the list and
      # new entries are always added to the bottom. This ensures that dependent
      # entries will be installed prior to the package with the dependency
      # otherwise install hooks may fail if they contain logic that depend on
      # the dependency.
      read -r -a pkg_build_tdeps_resolved <<<"$(_add_dep_to_tdep_list "$tdep" "${pkg_build_tdeps_resolved[@]}")"
    done
  done
}

# **Internal** Loads a Scaffolding package if `$pkg_scaffolding` is set. If the
# Scaffolding package's implementation contains a `scaffolding_load()`
# function, it is executed here so that the package can further influence the
# run and build dependencies of the Plan.
_load_scaffolding() {
  local lib
  if [[ -z "${pkg_scaffolding:-}" ]]; then
    return 0
  fi

  lib="$(_pkg_path_for_build_deps "$pkg_scaffolding")/lib/scaffolding.sh"
  build_line "Loading Scaffolding $lib"
  if ! source "$lib"; then
    exit_with "Failed to load Scaffolding from $lib" 17
  fi

  if [[ "$(type -t scaffolding_load)" == "function" ]]; then
    scaffolding_load
  fi
}

# **Internal** Determines suitable package identifiers for each run
# dependency and populates several package-related arrays for use throughout
# this program.
#
# Walk each item in $pkg_deps`, and for each item determine the absolute path
# to a suitable package release (which will be on disk).
_resolve_run_dependencies() {
  build_line "Resolving run dependencies"
  local resolved
  local dep
  local tdep
  local tdeps

  # Append to `${pkg_deps_resolved[@]}` all resolved direct run dependencies.
  for dep in "${pkg_deps[@]}"; do
    _install_dependency "$dep" "--ignore-install-hook"
    if resolved="$(_resolve_dependency "$dep")"; then
      build_line "Resolved dependency '$dep' to $resolved"
      pkg_deps_resolved+=("$resolved")
    else
      exit_with "Resolving '$dep' failed, should this be built first?" 1
    fi
  done

  # Append to `${pkg_tdeps_resolved[@]}` all the direct run dependencies, and
  # the run dependencies for each direct run dependency.

  # Copy all direct dependencies into a new array
  pkg_tdeps_resolved=("${pkg_deps_resolved[@]}")
  # Append all non-direct (transitive) run dependencies for each direct run
  # dependency. Any duplicate entries are dropped to produce a proper set.
  for dep in "${pkg_deps_resolved[@]}"; do
    mapfile -t tdeps < <(_get_tdeps_for "$dep") # See syntax note @ _get_tdeps_for
    for tdep in "${tdeps[@]}"; do
      tdep="$HAB_PKG_PATH/$tdep"
      # Use _add_dep_to_tdep_list instead of _return_or_append_to_set
      # so that duplicate entries are removed from the top of the list and
      # new entries are always added to the bottom. This ensures that dependent
      # entries will be installed prior to the package with the dependency
      # otherwise install hooks may fail if they contain logic that depend on
      # the dependency.
      read -r -a pkg_tdeps_resolved <<<"$(_add_dep_to_tdep_list "$tdep" "${pkg_tdeps_resolved[@]}")" # See syntax note @ _return_or_append_to_set
    done
  done
}

# **Internal** Populates the remaining package-related arrays used throughout
# this program.
_populate_dependency_arrays() {
  local dep

  # Build `${pkg_all_deps_resolved[@]}` containing all direct build and run
  # dependencies. The build dependencies appear before the run dependencies.
  pkg_all_deps_resolved=(
    "${pkg_build_deps_resolved[@]}"
    "${pkg_deps_resolved[@]}"
  )

  # Build an ordered set of all build and run dependencies (direct and
  # transitive). The order is important as this gets used when setting the
  # `$PATH` ordering in the build environment. To give priority to direct
  # dependencies over transitive ones the order of packages is the following:
  #
  # 1. All direct build dependencies
  # 2. All direct run dependencies
  # 3. All unique transitive build dependencies that aren't already added
  # 4. All unique transitive run dependencies that aren't already added
  pkg_all_tdeps_resolved=(
    "${pkg_build_deps_resolved[@]}"
    "${pkg_deps_resolved[@]}"
  )
  for dep in "${pkg_build_tdeps_resolved[@]}" "${pkg_tdeps_resolved[@]}"; do
    read -r -a pkg_all_tdeps_resolved <<<"$(_return_or_append_to_set "$dep" "${pkg_all_tdeps_resolved[@]}")" # See syntax note @ _return_or_append_to_set
  done
}

# **Internal** Validates that the computed dependencies are reasonable and that
# the full runtime set is unique--that is, there are no duplicate entries of
# the same `ORIGIN/NAME` tokens. An example would be a Plan which has a
# dependency on `acme/glibc` and a dependency on `acme/pcre` which uses an
# older version of `acme/glibc`. This leads to a package which would have 2
# version of `acme/glibc` in the shared library `RUNPATH` (`RPATH`). Rather
# than building a package which is destined to fail at runtime, this function
# will fast-fail with dependency information which an end user can use to
# resolve the situation before continuing.
_validate_deps() {
  # Build the list of full runtime deps (one per line) without the
  # `$HAB_PKG_PATH` prefix.
  local tdeps
  tdeps=$(echo "${pkg_tdeps_resolved[@]}" |
    tr ' ' '\n' |
    sed "s,^${HAB_PKG_PATH}/,,")
  # Build the list of any runtime deps that appear more than once. That is,
  # `ORIGIN/NAME` token duplicates.
  local dupes
  dupes=$(echo "$tdeps" |
    awk -F/ '{print $1"/"$2}' |
    sort |
    uniq -d)

  if [[ -n "$dupes" ]]; then
    local dupe
    # Build a list of all fully qualified package identifiers that are members
    # of the duplicated `ORIGIN/NAME` tokens. This will be used to star the
    # problematic dependencies in the graph.
    _dupes_qualified=$(echo "$tdeps" |
      grep -E "($(echo "$dupes" | tr '\n' '|' | sed 's,|$,,'))")

    warn
    warn "The following runtime dependencies have more than one version"
    warn "release in the full dependency chain:"
    warn
    echo "$dupes" | while read -r dupe; do
      warn "  * $dupe ( $(echo "$tdeps" | grep "^${dupe}/" | tr '\n' ' '))"
    done
    warn
    warn "The current situation usually arises when a Plan has a direct "
    warn "dependency on one version of a package (\`acme/A/7.0/20160101200001\`)"
    warn "and has a direct dependency on another package which itself depends"
    warn "on another version of the same package (\`acme/A/2.0/20151201060001\`)."
    warn "If this package (\`acme/A\`) contains shared libraries which are"
    warn "loaded at runtime by the current Plan, then both versions of"
    warn "\`acme/A\` could be loaded into the same process in a potentially"
    warn "surprising order. Worse, if both versions of \`acme/A\` are"
    warn "ABI-incompatible, runtime segmentation faults are more than likely."
    warn
    warn "In order to preserve reliability at runtime the duplicate dependency"
    warn "entries will need to be resolved before this Plan can be built."
    warn "Below is an expanded graph of all \`\$pkg_deps\` and their dependencies"
    warn "with the problematic lines noted."
    warn
    warn "Computed dependency graph (Lines with '*' denote a problematic entry):"
    echo -e "\n${pkg_origin}/${pkg_name}/${pkg_version}/${pkg_release}"
    echo "${pkg_deps_resolved[@]}" |
      tr ' ' '\n' |
      sed -e "s,^${HAB_PKG_PATH}/,," |
      _print_recursive_deps 1
    echo
    exit_with "Computed runtime dependency check failed, aborting" 31
  fi

  return 0
}

# **Internal** Prints a dependency graph in a format to the `tree(1)` command.
# This is used in concert with `_validate_deps` for the purpose of output to an
# end user.  It accepts a standard in stream as input where each line is a
# direct dependency package identifier of some package. The first function
# parameter is the leading padding depth when printing the dependency line.
# Finally, a global internal variable, `$_dupes_qualified`, is used to display
# which dependency entries have the duplicate versions present. An example
# should help to clarify:
#
# ```
# _dupes_qualified=$(cat <<EOF
# acme/glibc/2.22/20160309153915
# acme/glibc/2.22/20160308150809
# acme/linux-headers/4.3/20160309153535
# acme/linux-headers/4.3/20160308150438
# EOF
# )
#
# echo "acme/less/481/20160309165238"
#
# cat <<EOF | _print_recursive_deps 1
# acme/glibc/2.22/20160309153915
# acme/ncurses/6.0/20160308165339
# acme/pcre/8.38/20160308165506
# EOF
# ```
#
# And the corresponding output, in this case showing the problematic
# dependencies:
#
# ```
# acme/less/481/20160309165238
#     acme/glibc/2.22/20160309153915 (*)
#         acme/linux-headers/4.3/20160309153535 (*)
#     acme/ncurses/6.0/20160308165339
#         acme/glibc/2.22/20160308150809 (*)
#             acme/linux-headers/4.3/20160308150438 (*)
#         acme/gcc-libs/5.2.0/20160308165030
#             acme/glibc/2.22/20160308150809 (*)
#                 acme/linux-headers/4.3/20160308150438 (*)
#     acme/pcre/8.38/20160308165506
#         acme/glibc/2.22/20160308150809 (*)
#             acme/linux-headers/4.3/20160308150438 (*)
#         acme/gcc-libs/5.2.0/20160308165030
#             acme/glibc/2.22/20160308150809 (*)
#                 acme/linux-headers/4.3/20160308150438 (*)
# ```
_print_recursive_deps() {
  local level=$1
  local dep
  # Compute the amount of leading whitespace when display this line and any
  # child dependencies.
  local padn=$((level * 4))
  while read -r dep; do
    # If this dependency is a member of the duplicated set, then add an
    # asterisk at the end of the line, otherwise print the dependency.
    if echo "$_dupes_qualified" | grep -q "$dep" >/dev/null; then
      printf "%*s$dep (*)\n" $padn
    else
      printf "%*s$dep\n" $padn
    fi
    # If this dependency itself has direct dependencies, then recursively print
    # them.
    if [[ -f $HAB_PKG_PATH/$dep/DEPS ]]; then
      _print_recursive_deps $((level + 1)) <"$HAB_PKG_PATH"/"$dep"/DEPS
    fi
  done
}

# **Internal** Returns the path for the desired build package dependency
# on stdout from the resolved dependency set. Note that this function will
# only look for resolved build dependencies--runtime dependencies are not
# included in search.
#
# ```
# pkg_build_deps_resolved=(
#   /hab/pkgs/acme/zlib/1.2.8/20151216221001
#   /hab/pkgs/acme/nginx/1.8.0/20150911120000
#   /hab/pkgs/acme/glibc/2.22/20151216221001
# )
#
# _pkg_path_for_build_deps acme/nginx
# # /hab/pkgs/acme/nginx/1.8.0/20150911120000
# _pkg_path_for_build_deps zlib
# # /hab/pkgs/acme/zlib/1.2.8/20151216221001
# _pkg_path_for_build_deps glibc/2.22
# # /hab/pkgs/acme/glibc/2.22/20151216221001
# ```
#
# Will return 0 if a package is found locally on disk, and 1 if a package
# cannot be found. A message will be printed to stderr to provide context.
_pkg_path_for_build_deps() {
  local dep="$1"
  local e
  local cutn="$(($(echo "$HAB_PKG_PATH" | grep -o '/' | wc -l) + 2))"
  for e in "${pkg_build_deps_resolved[@]}"; do
    if echo "$e" | cut -d "/" -f ${cutn}- | grep -E -q "(^|/)${dep}(/|$)"; then
      echo "$e"
      return 0
    fi
  done
  return 1
}

# **Internal** Returns the path for the desired runtime package dependency
# on stdout from the resolved dependency set. Note that this function will
# only look for resolved runtime dependencies--build dependencies are not
# included in search.
#
# ```
# pkg_deps_resolved=(
#   /hab/pkgs/acme/zlib/1.2.8/20151216221001
#   /hab/pkgs/acme/nginx/1.8.0/20150911120000
#   /hab/pkgs/acme/glibc/2.22/20151216221001
# )
#
# _pkg_path_for_deps acme/nginx
# # /hab/pkgs/acme/nginx/1.8.0/20150911120000
# _pkg_path_for_deps zlib
# # /hab/pkgs/acme/zlib/1.2.8/20151216221001
# _pkg_path_for_deps glibc/2.22
# # /hab/pkgs/acme/glibc/2.22/20151216221001
# ```
#
# Will return 0 if a package is found locally on disk, and 1 if a package
# cannot be found. A message will be printed to stderr to provide context.
_pkg_path_for_deps() {
  local dep="$1"
  local e
  local cutn="$(($(echo "$HAB_PKG_PATH" | grep -o '/' | wc -l) + 2))"
  for e in "${pkg_deps_resolved[@]}"; do
    if echo "$e" | cut -d "/" -f ${cutn}- | grep -E -q "(^|/)${dep}(/|$)"; then
      echo "$e"
      return 0
    fi
  done
  if [[ "${FUNCNAME[1]}" != "pkg_interpreter_for" ]]; then
    warn "No runtime dependency found for '$dep'"
    warn "Resolved runtime package set: ${pkg_deps_resolved[*]}"
  fi
  return 1
}

# ## Public helper functions
#
# These functions intended for use in this program and by Plan authors in their
# `plan.sh` files.

# **Internal** Downloads, resolves, and normalizes all build and run
# dependencies. If Scaffolding is being used, this function also injects the
# relevant packages into the build dependencies and allows Scaffolding packages
# to further mutate the run dependencies for the Plan.
#
# Several package-related arrays are created as a result:

# * `$pkg_build_deps_resolved`: A package-path array of all direct build
#    dependencies, declared in `$pkg_build_deps`.
# * `$pkg_build_tdeps_resolved`: A package-path array of all direct build
#    dependencies and the run dependencies for each direct build dependency.
# * `$pkg_deps_resolved`: A package-path array of all direct run dependencies,
#    declared in `$pkg_deps`.
# * `$pkg_tdeps_resolved`:  A package-path array of all direct run dependencies
#    and the run dependencies for each direct run dependency.
# * `$pkg_all_deps_resolved`: A package-path array of all direct build and
#    run dependencies, declared in `$pkg_build_deps` and `$pkg_deps`.
# * `$pkg_all_tdeps_resolved`: An ordered package-path array of all direct
#    run and build dependencies, and the run dependencies for each direct
#    dependency. Further details in the `_populate_dependency_arrays()`
#    function.
_resolve_dependencies() {
  # Create initial package arrays
  _init_dependencies

  # Inject, download, and resolve the scaffolding dependencies
  _resolve_scaffolding_dependencies

  # Populate package arrays to enable helper functions for early scaffolding
  # load hooks
  _populate_dependency_arrays

  # Load scaffolding packages if they are being used.
  _load_scaffolding

  # Download and resolve the build dependencies
  _resolve_build_dependencies

  # Download and resolve the run dependencies
  _resolve_run_dependencies

  # Finalize and normalize all resolved dependencies with all build and run
  # dependencies
  _populate_dependency_arrays

  # Validate the dependency arrays
  _validate_deps
}

