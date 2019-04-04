#!/bin/bash
#
# # Usage
#
# ```
# $ hab-plan-build [<PATH>]
# ```
#
# # Synopsis
#
# `hab-plan-build` handles creating Habitat packages from Plan files.
#
# # plan.sh
#
# The heart of `hab-plan-build` is the Plan. This file is a shell
# script which defines how you download, configure, make, install,
# and run software. Think of it as a cross between an rpm `spec` file
# and a `Dockerfile`.
#
# Here is an example `plan.sh` for [zlib](http://www.zlib.net/) - it
# also happens to be the smallest possible `plan.sh`:
#
# ```sh
# pkg_name=zlib
# pkg_description="The zlib library"
# pkg_upstream_url=http://zlib.net
# pkg_maintainer="Frank Lloyd Wright <flw@example.com>"
# pkg_version=1.2.8
# pkg_license=('zlib')
# pkg_source=http://downloads.sourceforge.net/project/libpng/$pkg_name/${pkg_version}/${pkg_name}-${pkg_version}.tar.gz
# pkg_filename=${pkg_name}-${pkg_version}.tar.gz
# pkg_shasum=36658cb768a54c1d4dec43c3116c27ed893e88b02ecfcb44f2166f9c0b7f2a0d
# pkg_deps=(glibc)
# pkg_lib_dirs=(lib)
# pkg_include_dirs=(include)
# ```
#
# It has the name of the software, the version, where to download it, a
# checksum to verify the contents are what we expect, an origin key to sign the
# resulting package with, a single dependency on glibc, and it has the
# resulting libraries in `lib` and header files in `include`.
#
# When executed, this program will:
#
# 1. Download the software
# 1. Validate the checksum
# 1. Uncompress it
# 1. Set the build environment to depend on the `glibc` package
# 1. Run `./configure && make`
# 1. Run `make install`
# 1. Write out the data other packages need to depend on `zlib`
# 1. Create a libsodium-signed tarball of the results
#
# ## Plan Options
#
# ### pkg_name
# Sets the name of the package.
# ```
# pkg_name=zlib
# ```
#
# ### pkg_description
# A short description of the package.
# ```
# pkg_description="The zlib library"
# ```
# This is rendered as markdown, so use a bash heredoc to insert a block of text that will be displayed on the web site. Be careful with escaping special characters, it's bash after all!
#```
# pkg_description=$(cat <<EOF
# This is a multiline description.
# \`\`\`
# % echo "Maybe it has code examples?"
# \`\`\`
# Build a happy Habitat!
# EOF
# )
#```
#
# ### pkg_upstream_url
# The URL to the upstream project's website or home page.
# ```
# pkg_upstream_url=http://zlib.net
# ```
#
# ### pkg_maintainer
# The name and email address of the Plan and package maintainer (not necessarily the maintainer of the upstream project).
# ```
# pkg_maintainer="Frank Llyod Wright <flw@example.com>"
# ```
#
# ### pkg_version
# Sets the version of the package.
# ```
# pkg_version=1.2.8
# ```
#
# ### pkg_license
# An array of software license that relate to this package. Used in the
# MANIFEST.
# ```
# pkg_license=('zlib')
# ```
#
# ### pkg_source
# Where to download an external source from. Any valid `wget` url will work. If
# the source is local to the `plan.sh`, then omit this value.
# ```
# pkg_source=http://downloads.sourceforge.net/project/libpng/$pkg_name/${pkg_version}/${pkg_name}-${pkg_version}.tar.gz
# ```
#
# ### pkg_filename
# The resulting filename for the download.
# ```
# pkg_filename=${pkg_name}-${pkg_version}.tar.gz
# ```
#
# ### pkg_shasum
# The sha256 sum of for the externally downloaded `$pkg_source`, if used. You
# can easily generate by downloading the source and using `sha256sum` or
# `gsha256sum`.
# ```
# pkg_shasum=36658cb768a54c1d4dec43c3116c27ed893e88b02ecfcb44f2166f9c0b7f2a0d
# ```
#
#
# ### pkg_deps
# An array of the package dependencies needed at runtime.
# ```
# pkg_deps=(glibc pcre openssl zlib)
# ```
#
# ### pkg_build_deps
# An array of the package dependencies needed only at build time.
# ```
# pkg_build_deps=(linux-headers)
# ```
#
# ### pkg_lib_dirs
# An array of paths, relative to the final install of the software, where
# libraries can be found. Used to populate `LD_FLAGS` and `LD_RUN_PATH` for
# software that depends on your package.
# ```
# pkg_lib_dirs=(lib)
# ```
#
# ### pkg_include_dirs
# An array of paths, relative to the final install of the software, where
# headers can be found. Used to populate `CFLAGS`, `CPPFLAGS` and `CXXFLAGS`
# for software that depends on your package.
# ```
# pkg_include_dirs=(include)
# ```
#
# ### pkg_bin_dirs
# An array of paths, relative to the final install of the software, where
# binaries can be found. Used to populate `PATH` for software that depends on
# your package.
# ```
# pkg_bin_dirs=(bin)
# ```
#
# ### pkg_pconfig_dirs
# An array of paths, relative to the final install of the software,
# where pkg-config metadata (.pc files) can be found.  Used to populate
# PKG_CONFIG_PATH for software that depends on your package.
# ```
# pkg_pconfig_dirs=(lib/pkgconfig)
# ```
#
# ### pkg_svc_run
# The command to start the service, if needed. Should not fork!
# ```
# pkg_svc_run="haproxy -f $pkg_svc_config_path/haproxy.conf"
# ```
#
# ### pkg_exports
# An associative array representing configuration data which should be gossiped to peers. The keys
# in this array represent the name the value will be assigned and the values represent the toml path
# to read the value.
# ```
# pkg_exports=(
#   [key]=token
#   [port]=server.port
#   [host]=server.host
# )
# ```
#
# ### pkg_exposes
# An array of `pkg_exports` keys containing default values for which ports that this package
# exposes. These values are used as sensible defaults for other tools. For example, when exporting
# a package to a container format.
# ```
# pkg_exposes=(port)
# ```
#
# ### pkg_binds
# An associative array representing services which you depend on and the configuration keys that
# you expect the service to export (by their `pkg_exports`). These binds *must* be set for the
# Supervisor to load the service. The loaded service will wait to run until its bind becomes
# available. If the bind does not contain the expected keys, the service will not start
# successfully.
# ```
# pkg_binds=(
#   [database]="port host"
# )
#
# ### pkg_binds_optional
# Same as `pkg_binds` but these represent optional services to connect to.
# ```
# pkg_binds_optional=(
#   [storage]="port host"
# )
#
# ### pkg_origin
# A string to use for the origin. The origin is used to denote a particular upstream of a
# package; when we resolve dependencies, we consider a version of a package to be equal
# regardless of its origin - but you can specify what you prefer to use.
# ```
# pkg_origin=acme
# ```
#
# ### pkg_interpreters
# An array of interpreters used in shebang lines for scripts. Specify
# the subdirectory where the binary is relative to the package, e.g.,
# `bin/bash` or `libexec/neverland`, since binaries can be located in
# directories besides `bin`. This list of interpreters will be written
# to the metadata INTERPRETERS file with their fully-qualified path.
# Then these can be used with the `fix_interpreter` function in this script.
# ```
# pkg_interpreters=(bin/bash bin/sh)
# ```
#
# ## Plan variables
#
# `hab-plan-build` sets a few useful variables for you, in addition to the ones
# you define above. They are:
#
# * `$pkg_prefix`: This variable is the final path for your package.
# * `$pkg_dirname`: Set to `${pkg_name}-${pkg_version}` by default
# * `$pkg_svc_path`: Where the running service is; `$HAB_ROOT_PATH/svc/$pkg_name`
# * `$pkg_svc_data_path`: Service data; `$pkg_svc_path/data`
# * `$pkg_svc_files_path`: Gossiped config files; `$pkg_svc_path/files`
# * `$pkg_svc_var_path`: Variable state; `$pkg_svc_path/var`
# * `$pkg_svc_config_path`: Configuration; `$pkg_svc_path/config`
# * `$pkg_svc_static_path`: Static content; `$pkg_svc_path/static`
# * `$HAB_CACHE_SRC_PATH`: The path to all the externally downloaded package
#      sources
# * `$HAB_CACHE_ARTIFACT_PATH`: The default download root path for package
#      artifacts, used on package installation
# * `$HAB_CACHE_KEY_PATH`: The path where cryptographic keys are stored
# * `$CFLAGS`: C compiler options
# * `$LDFLAGS`: C linker options
# * `$PREFIX`: Where to install the software; same as $pkg_prefix
# * `$LD_RUN_PATH`: Where to find binaries at run time
#
# ## Plan Callbacks (Build Phases)
#
# `hab-plan-build` allows you to alter its behavior by defining callbacks (or build phases)
# within the `plan.sh`. While `zlib` is awesome and requires no callbacks, most
# software isn't quite so simple to build. To define a callback, simply create
# a shell function with its name - then write out your script.
#
# Most likely, you simply need to redefine two steps:
#
# 1. do_build - this step runs './configure --prefix=$pkg_prefix && make`.
# 1. do_install - this step runs `make install`.
#
# Here is an example `plan.sh` for `haproxy`, which overrides both:
#
# ```sh
# pkg_name=haproxy
# pkg_version=1.5.12
# pkg_license=('BSD')
# pkg_maintainer="Frank Lloyd Wright <flw@example.com>"
# pkg_source=http://www.haproxy.org/download/1.5/src/${pkg_name}-${pkg_version}.tar.gz
# pkg_filename=${pkg_name}-${pkg_version}.tar.gz
# pkg_shasum=6648dd7d6b958d83dd7101eab5792178212a66c884bec0ebcd8abc39df83bb78
# pkg_bin_dirs=(bin)
# pkg_deps=(glibc pcre openssl zlib)
# pkg_svc_run="haproxy -f $pkg_svc_config_path/haproxy.conf"
# pkg_exposes=(port)
# pkg_exports=(
#   [mode]=mode
#   [port]=server.port
# )
#
# do_build() {
#   make USE_PCRE=1 \
#     USE_PCRE_JIT=1 \
#     CPU=x86_64 \
#     TARGET=linux2628 \
#     USE_OPENSSL=1 \
#     USE_ZLIB=1 \
#     ADDINC="$CFLAGS" \
#     ADDLIB="$LDFLAGS"
# }
#
# do_install() {
#   mkdir -p $pkg_prefix/bin
#   cp haproxy $pkg_prefix/bin
# }
# ```
#
# [A complete list of callbacks, their description, and attendant source code,
# is but a click away.](#build-phases)
#
# # License and Copyright
# ```
# Copyright: Copyright (c) 2016-2017 Chef Software, Inc.
# License: Apache License, Version 2.0
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
# ```
#
#

# # Internals
source_dir=$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )
source "${source_dir}/public.bash"
source "${source_dir}/shared.bash"
source "${source_dir}/environment.bash"

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
# The current version of this program
HAB_PLAN_BUILD=0.0.1
# The root path of the Habitat file system. If the `$HAB_ROOT_PATH` environment
# variable is set, this value is overridden, otherwise it is set to its default
: "${HAB_ROOT_PATH:=/hab}"
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
FALLBACK_CHANNEL="stable"
# The value of `$PATH` on initial start of this program
INITIAL_PATH="$PATH"
# The value of `pwd` on initial start of this program
INITIAL_PWD="$(pwd)"
# The compression level to use when compression harts (0..9)
: "${HAB_HART_COMPRESSION_LEVEL:=6}"
# The full target tuple this plan will be built for
pkg_target='@@pkg_target@@'
# The package's origin (i.e. acme)
pkg_origin=""
# Each release is a timestamp - `YYYYMMDDhhmmss`
pkg_release=$(date -u +%Y%m%d%H%M%S)
# The default build deps setting - an empty array
pkg_build_deps=()
# The default runtime deps setting - an empty array
pkg_deps=()
# The path inside a package that contains libraries - used in `LD_RUN_PATH` and
# `LD_FLAGS`.
pkg_lib_dirs=()
# The path inside a package that contains binary programs - used in `PATH`
pkg_bin_dirs=()
# The path inside a package that contains header files - used in `CFLAGS`
pkg_include_dirs=()
# The path(s) inside a package that contain pkg-config (.pc) files
pkg_pconfig_dirs=()
# The command to run the service - must not fork or return
pkg_svc_run=''
pkg_exposes=()
declare -A pkg_exports
declare -A pkg_binds
declare -A pkg_binds_optional
# The user to run the service as
pkg_svc_user=hab
# The group to run the service as
pkg_svc_group=$pkg_svc_user

# Initially set $pkg_svc_* variables. This happens before the Plan is sourced,
# meaning that `$pkg_name` is not yet set. However, `$pkg_svc_run` wants
# to use these variables, so what to do? We'll set up these svc variables
# with the `$pkg_svc_run` variable as the customer-in-mind and pass over
# it once the Plan has been loaded. For good measure, all of these variables
# will need to be set again.
pkg_svc_path="$HAB_ROOT_PATH/svc/@__pkg_name__@"
pkg_svc_data_path="$pkg_svc_path/data"
pkg_svc_files_path="$pkg_svc_path/files"
pkg_svc_var_path="$pkg_svc_path/var"
pkg_svc_config_path="$pkg_svc_path/config"
pkg_svc_config_install_path="$pkg_svc_path/config_install"
pkg_svc_static_path="$pkg_svc_path/static"

# Used to handle if we received a signal, or failed based on a bad status code.
graceful_exit=true

# We want everything to be build as `rwx-r-x-r-x`
umask 0022

# In order to ensure that the external environment does not affect the build
# time behavior of a Plan, we explicitly unset several know environment
# variables which are known to be used by underlying autoconf-like tools
# and other build software.
unset TERMINFO

_artifact_ext="hart"

# ## Private/Internal helper functions
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
    echo "   ${pkg_name}: Build time: $elapsed"
  else
    case "${TERM:-}" in
      *term | xterm-* | rxvt | screen | screen-*)
        echo -e "   \033[1;36m${pkg_name}: \033[1;37mBuild time: $elapsed\033[0m"
        ;;
      *)
        echo "   ${pkg_name}: Build time: $elapsed"
        ;;
    esac
  fi
  if [[ $exit_status -ne 0 ]]; then
    _build_error_message "Exiting on error"

    # If the user has set up a failure hook, execute it now
    do_after_failure_wrapper
  fi

  # If we don't unset the trap for EXIT, we can end up double-exiting,
  # which could cause a `do_after_failure` callback to fire twice.
  trap - EXIT
  exit "$exit_status"
}

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

# **Internal**  Build a `PATH` string suitable for entering into this package's
# `RUNTIME_PATH` metadata file. The ordering of this path is important as this
# value will ultimately be consumed by other programs such as the Supervisor
# when constructing the `PATH` environment variable before spawning a process.
#
# The path is constructed by taking all `PATH` metadata file entries from this
# package (in for the form of `$pkg_bin_dirs[@]`), followed by entries from the
# *direct* dependencies first (in declared order), and then from any remaining
# transitive dependencies last (in lexically sorted order). All entries are
# present only once in the order of their first appearance.
_assemble_runtime_path() {
  local paths=()
  local dir dep data

  # Add element for each entry in `$pkg_bin_dirs[@]` first
  for dir in "${pkg_bin_dirs[@]}"; do
    paths+=("$pkg_prefix/$dir")
  done

  # Iterate through all direct direct run dependencies following by all
  # remaining transitive run dependencies and for each, append each path entry
  # onto the result, assuming it hasn't already been added. In this way, all
  # direct dependencies will match first and any programs that are used by a
  # direct dependency will also be present on PATH, albeit at the very end of
  # the PATH. Additionally, any path entries that don't relate to the
  # dependency in question are filtered out to deal with a vintage of packages
  # which included more data in `PATH` and have since been addressed.
  for dep_prefix in "${pkg_deps_resolved[@]}" "${pkg_tdeps_resolved[@]}"; do
    if [[ -f "$dep_prefix/PATH" ]]; then
      data="$(cat "$dep_prefix/PATH")"
      data="$(trim "$data")"
      while read -r entry; do
        read -r -a paths <<< "$(_return_or_append_to_set "$entry" "${paths[@]}")" # See syntax note @ _return_or_append_to_set
      done <<< "$(echo "$data" | tr ':' '\n' | grep "^$dep_prefix")"
    fi
  done

  # Return the elements of the result, joined with a colon
  join_by ':' "${paths[@]}"
}

_ensure_origin_key_present() {
  local cache="$HAB_CACHE_KEY_PATH"
  local keys_found
  keys_found="$(find $cache -name "${pkg_origin}-*.sig.key" | wc -l)"
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
# * `$_wget_cmd` (wget on system)
# * `$_shasum_cmd` (either gsha256sum or sha256sum on system)
# * `$_tar_cmd` (GNU version of tar)
# * `$_mktemp_cmd` (GNU version from coreutils)
# * `$_rq_cmd`
#
# Note that all of the commands noted above are considered internal
# implementation details and are subject to change with little to no notice,
# which means the variables such as `$_tar_cmd` should **not** be used directly
# by Plan authors. The leading underscore denotes an internal/private variable
# or function.

# If the commands are not found, `exit_with` is called and the program is
# terminated.
_find_system_commands() {
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

  if exists rq; then
    _rq_cmd=$(command -v rq)
  else
    exit_with "We required rq to build package metadata; aborting" 1
  fi
  debug "Setting _rq_cmd=$_rq_cmd"
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
  if result="$($HAB_BIN pkg path "$1" 2> /dev/null)"; then
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
  if ! echo "$dep" | grep -q '\/' > /dev/null; then
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
    $HAB_BIN install -u $HAB_BLDR_URL --channel $HAB_BLDR_CHANNEL ${IGNORE_LOCAL:-} "$@" || {
      if [[ "$HAB_BLDR_CHANNEL" != "$FALLBACK_CHANNEL" ]]; then
        build_line "Trying to install '$dep' from '$FALLBACK_CHANNEL'"
        $HAB_BIN install -u $HAB_BLDR_URL --channel "$FALLBACK_CHANNEL" ${IGNORE_LOCAL:-} "$@" || true
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

# **Internal** Prints the source file, line number, and lines of context around
# the current debugging session context. Used by `attach()` and should not be
# used externally.
#
# ```
# _attach_whereami    # => defaults to 10 lines of context around the current line
# _attach_whereami 2  # => shows 2 lines of context around the current line
# ```
_attach_whereami() {
  local context=${1:-10}
  local lnum="${BASH_LINENO[1]}"
  local src="${BASH_SOURCE[2]}"
  # If we are printing this program, use the absolute path version
  if [[ "$src" = "$0" ]]; then
    src="$THIS_PROGRAM"
  fi
  echo
  echo "From: $src @ line $lnum :"
  echo
  awk '{printf "%d: %s\n", NR, $0}' "$src" \
    | sed -e "$((lnum - context)),$((lnum + context))!d" \
      -e 's,^,    ,g' \
    | sed -e "$((context +1))s/^   / =>/"
  echo
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

  if [[ -n "${HAB_BIN:-}" ]]; then
    HAB_BIN=$HAB_BIN
  else
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
      read -r -a pkg_build_tdeps_resolved <<< "$(_return_or_append_to_set "$tdep" "${pkg_build_tdeps_resolved[@]}")" # See syntax note @ _return_or_append_to_set
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
    if [[ -n "${HAB_FEAT_INSTALL_HOOK:-}" ]]; then
      _install_dependency "$dep" "--ignore-install-hook"
    else
      _install_dependency "$dep"
    fi
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
      read -r -a pkg_tdeps_resolved <<< "$(_return_or_append_to_set "$tdep" "${pkg_tdeps_resolved[@]}")" # See syntax note @ _return_or_append_to_set
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
    "${pkg_deps_resolved[@]}"
    "${pkg_build_deps_resolved[@]}"
  )

  # Build an ordered set of all build and run dependencies (direct and
  # transitive). The order is important as this gets used when setting the
  # `$PATH` ordering in the build environment. To give priority to direct
  # dependencies over transitive ones the order of packages is the following:
  #
  # 1. All direct run dependencies
  # 2. All direct build dependencies
  # 3. All unique transitive run dependencies that aren't already added
  # 4. All unique transitive build dependencies that aren't already added
  pkg_all_tdeps_resolved=(
    "${pkg_deps_resolved[@]}"
    "${pkg_build_deps_resolved[@]}"
  )
  for dep in "${pkg_tdeps_resolved[@]}" "${pkg_build_tdeps_resolved[@]}"; do
    read -r -a pkg_all_tdeps_resolved <<< "$(_return_or_append_to_set "$dep" "${pkg_all_tdeps_resolved[@]}")" # See syntax note @ _return_or_append_to_set
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
  tdeps=$(echo "${pkg_tdeps_resolved[@]}" \
    | tr ' ' '\n' \
    | sed "s,^${HAB_PKG_PATH}/,,")
  # Build the list of any runtime deps that appear more than once. That is,
  # `ORIGIN/NAME` token duplicates.
  local dupes
  dupes=$(echo "$tdeps" \
    | awk -F/ '{print $1"/"$2}' \
    | sort \
    | uniq -d)

  if [[ -n "$dupes" ]]; then
    local dupe
    # Build a list of all fully qualified package identifiers that are members
    # of the duplicated `ORIGIN/NAME` tokens. This will be used to star the
    # problematic dependencies in the graph.
    _dupes_qualified=$(echo "$tdeps" \
      | grep -E "($(echo "$dupes" | tr '\n' '|' | sed 's,|$,,'))")

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
    echo "${pkg_deps_resolved[@]}" \
      | tr ' ' '\n' \
      | sed -e "s,^${HAB_PKG_PATH}/,," \
      | _print_recursive_deps 1
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
    if echo "$_dupes_qualified" | grep -q "$dep" > /dev/null; then
      printf "%*s$dep (*)\n" $padn
    else
      printf "%*s$dep\n" $padn
    fi
    # If this dependency itself has direct dependencies, then recursively print
    # them.
    if [[ -f $HAB_PKG_PATH/$dep/DEPS ]]; then
      _print_recursive_deps $((level + 1)) < "$HAB_PKG_PATH"/"$dep"/DEPS
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
  local cutn="$(($(echo $HAB_PKG_PATH | grep -o '/' | wc -l)+2))"
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
  local cutn="$(($(echo $HAB_PKG_PATH | grep -o '/' | wc -l)+2))"
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



# **Internal** Convert a string into a numerical value.
_to_int() {
    local -i num="10#${1}"
    echo "${num}"
}

# **Internal** Return 0 if the given value is a valid port and 1 if not.
#
# ```
# _port_is_valid "80"
# # 0
# _port_is_valid 80
# # 0
# _port_is_valid "hello"
# # 1
# ```
_port_is_valid() {
    local port="$1"
    local -i port_num
    port_num=$(_to_int "$port" 2>/dev/null)
    if (( port_num < 1 || port_num > 65535 )) ; then
        return 1
    fi
    return 0
}

# ## Build Phases
#
# Stub build phases, in the order they are executed. These can be overridden by
# the `plan.sh` if needed.


# Used to execute arbitrary commands before anything else happens. Delegates
# most implementation to the `do_default_begin()` function.
do_begin() {
  do_default_begin
  return $?
}

# Default implementation for the `do_begin()` phase.
do_default_begin() {
  return 0
}

# **Internal** Downloads, resolves, and normalizes all build and run
# dependencies. If Scaffolding is being used, this function also injects the
# relevant packages into the build dependencies and allows Scaffolding packages
# to further mutate the run dependencies for the Plan.
#
# Several package-related arrays are created as a result:
#
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

# **Internal**  Build and export `$PATH` containing each path in our own
# `${pkg_bin_dirs[@]}` array, and then any dependency's `PATH` entry (direct or
# transitive) if one exists. The ordering of the path is specific to
# `${pkg_all_tdeps_resolved[@]}` which is further explained in the
# `_resolve_dependencies()` function.
#
# Reference implementation:
# https://github.com/habitat-sh/habitat/blob/3d63753468ace168bbbe4c52e600d408c4981b03/components/plan-build/bin/hab-plan-build.sh#L1584-L1638
_set_build_path() {
  local paths=()
  local dir dep data

  # Add element for each entry in `$pkg_bin_dirs[@]` first
  for dir in "${pkg_bin_dirs[@]}"; do
    paths+=("$pkg_prefix/$dir")
  done

  # Iterate through all build and run dependencies in the order present in
  # `${pkg_all_tdeps_resolved[@]}` and for each, append each path entry onto
  # the result, assuming it hasn't already been added. Additionally, any path
  # entries that don't relate to the dependency in question are filtered out to
  # deal with a vintage of packages which included more data in `PATH` and have
  # since been addressed.
  for dep_prefix in "${pkg_all_tdeps_resolved[@]}"; do
    if [[ -f "$dep_prefix/PATH" ]]; then
      data="$(cat "$dep_prefix/PATH")"
      data="$(trim "$data")"
      while read -r entry; do
        read -r -a paths <<< "$(_return_or_append_to_set "$entry" "${paths[@]}")" # See syntax note @ _return_or_append_to_set
      done <<< "$(echo "$data" | tr ':' '\n' | grep "^$dep_prefix")"
    fi
  done

  paths+=("$INITIAL_PATH")

  PATH="$(join_by ':' "${paths[@]}")"
  export PATH

  build_line "Setting PATH=$PATH"
}

# **Internal** This writes out a pre_build.env file, similar to the last_build.env
# that gets written at the end of a build. The purpose of pre_build.env is to provide
# metadata about what package we're trying to build before we build it. This is useful
# if the build fails, because a worker can read that file in and use it to report back
# about what exactly failed.
_write_pre_build_file() {
  local plan_owner
  plan_owner="$(stat -c '%u:%g' "$PLAN_CONTEXT/plan.sh")"
  pre_build_file="$pkg_output_path/pre_build.env"

  build_line "Writing pre_build file"

  mkdir -pv "$pkg_output_path"
  # Attempt to set user/group ownership to the same as the ownership of the
  # `plan.sh` file. If the `chown` fails, then don't stop the build--this is
  # only best effort.
  chown "$plan_owner" "$pkg_output_path" || true

  if [ -f "$pre_build_file" ]; then
    rm "$pre_build_file"
  fi

  cat <<-EOF > "$pre_build_file"
pkg_origin=$pkg_origin
pkg_name=$pkg_name
pkg_version=$pkg_version
pkg_release=$pkg_release
pkg_ident=${pkg_origin}/${pkg_name}/${pkg_version}/${pkg_release}
EOF

  chown "$plan_owner" "$pre_build_file" || true
}

# At this phase of the build, all dependencies are downloaded, the build
# environment is set, but this is just before any source downloading would
# occur (if `$pkg_source` is set). This could be a suitable phase in which to
# compute a dynamic version of a package given the state of a Git repository,
# fire an API call, start timing something, etc.
do_before() {
  do_default_before
  return $?
}

# Default implementation for the `do_before()` phase.
do_default_before() {
  return 0
}

# If `$pkg_source` is being used, download the software and place it in
# `$HAB_CACHE_SRC_PATH/$pkg_filename`. If the source already exists in the
# cache, verify that the checksum is what we expect, and skip the download.
# Delegates most of the implementation to the `do_default_download()` function.
do_download() {
  do_default_download
  return $?
}

# Default implementation for the `do_download()` phase.
do_default_download() {
  # If the source is local (and `$pkg_source` is not set) then return, nothing
  # to do
  if [[ -z "${pkg_source:-}" ]]; then
    return 0
  fi

  # shellcheck disable=2154
  download_file "$pkg_source" "$pkg_filename" "$pkg_shasum"
}

# If `$pkg_source` is being used, verify that the package we have in
# `$HAB_CACHE_SRC_PATH/$pkg_filename` has the `$pkg_shasum` we expect.
# Delegates most of the implementation to the `do_default_verify()` function.
do_verify() {
  do_default_verify
  return $?
}

# Default implementation for the `do_verify()` phase.
do_default_verify() {
  if [[ -n "${pkg_filename:-}" ]]; then
    verify_file "$pkg_filename" "$pkg_shasum"
  fi
}

# Clean up the remnants of any previous build job, ensuring it can't pollute
# out new output. Delegates most of the implementation to the
# `do_default_clean()` function.
do_clean() {
  do_default_clean
  return $?
}

# Default implementation for the `do_clean()` phase.
do_default_clean() {
  build_line "Clean the cache"
  rm -rf "$CACHE_PATH"
  return 0
}

# If `$pkg_source` is being used, we take the
# `$HAB_CACHE_SRC_PATH/$pkg_filename` from the download step and unpack it,
# as long as the method of extraction can be determined.
#
# This takes place in the $HAB_CACHE_SRC_PATH directory.
#
# Delegates most of the implementation to the `do_default_unpack()` function.
do_unpack() {
  do_default_unpack
  return $?
}

# Default implementation for the `do_unpack()` phase.
do_default_unpack() {
  if [[ -n "${pkg_filename:-}" ]]; then
    unpack_file "$pkg_filename"
  fi
}

# **Internal** Set up our build environment. First, add any library paths
# defined in `$pkg_lib_dirs` to `LD_RUN_PATH`. Then, for each dependency in
# `$pkg_deps`, and `$pkg_build_deps`, find the latest package, then add it's
# `LD_RUN_PATH`, `CFLAGS`, `LDFLAGS`, and `PATH` to ours. Also, set
# `PREFIX=$pkg_prefix`, ensuring that most software will install into the
# correct location.
_build_environment() {
  build_line "Setting build environment"

  # Build `$LD_RUN_PATH` containing each path in our own `${pkg_lib_dirs[@]}`
  # array and then each direct run dependency's `LD_RUN_PATH` entry if one
  # exists. This ensures that the resulting `RUNPATH` (like `RPATH`, but not
  # overridable) entries only contain **direct** **runtime** paths. If a
  # dependency's lib directory needs to be set in the resulting `RUNPATH`
  # sections of an ELF binary, it must be a direct dependency, not transitive.
  local ld_run_path_part=()
  for lib in "${pkg_lib_dirs[@]}"; do
    ld_run_path_part+=("$pkg_prefix/$lib")
  done
  for dep_path in "${pkg_deps_resolved[@]}"; do
    if [[ -f "$dep_path/LD_RUN_PATH" ]]; then
      local data
      data=$(cat "$dep_path"/LD_RUN_PATH)
      local trimmed
      trimmed=$(trim "$data")
      ld_run_path_part+=("$trimmed")
    fi
  done
  if [[ -n "${ld_run_path_part[*]}" ]]; then
    export LD_RUN_PATH
    LD_RUN_PATH=$(join_by ':' "${ld_run_path_part[@]}")
  fi

  # Build `$CFLAGS`, `$CPPFLAGS`, `$CXXFLAGS` and `$LDFLAGS` containing any
  # direct dependency's `CFLAGS`, `CPPFLAGS`, `CXXFLAGS` or `LDFLAGS` entries
  # respectively (build or run). If the software to be built requires the path
  # to headers or shared libraries, it must be a direct dependency, not
  # transitive.
  for dep_path in "${pkg_all_deps_resolved[@]}"; do
    if [[ -f "$dep_path/CFLAGS" ]]; then
      local data
      data=$(cat "$dep_path"/CFLAGS)
      local trimmed
      trimmed=$(trim "$data")
      if [[ -n "$CFLAGS" ]]; then
        export CFLAGS="$CFLAGS $trimmed"
      else
        export CFLAGS="$trimmed"
      fi
      if [[ ! -f "$dep_path/CPPFLAGS" ]]; then
        if [[ -n "$CPPFLAGS" ]]; then
          export CPPFLAGS="$CPPFLAGS $trimmed"
        else
          export CPPFLAGS="$trimmed"
        fi
      fi
      if [[ ! -f "$dep_path/CXXFLAGS" ]]; then
        if [[ -n "$CXXFLAGS" ]]; then
          export CXXFLAGS="$CXXFLAGS $trimmed"
        else
          export CXXFLAGS="$trimmed"
        fi
      fi
    fi

    if [[ -f "$dep_path/CPPFLAGS" ]]; then
      local data
      data=$(cat "$dep_path"/CPPFLAGS)
      local trimmed
      trimmed=$(trim "$data")
      if [[ -n "$CPPFLAGS" ]]; then
        export CPPFLAGS="$CPPFLAGS $trimmed"
      else
        export CPPFLAGS="$trimmed"
      fi
    fi

    if [[ -f "$dep_path/CXXFLAGS" ]]; then
      local data
      data=$(cat "$dep_path"/CXXFLAGS)
      local trimmed
      trimmed=$(trim "$data")
      if [[ -n "$CXXFLAGS" ]]; then
        export CXXFLAGS="$CXXFLAGS $trimmed"
      else
        export CXXFLAGS="$trimmed"
      fi
    fi

    if [[ -f "$dep_path/LDFLAGS" ]]; then
      local data
      data=$(cat "$dep_path"/LDFLAGS)
      local trimmed
      trimmed=$(trim "$data")
      if [[ -n "$LDFLAGS" ]]; then
        export LDFLAGS="$LDFLAGS $trimmed"
      else
        export LDFLAGS="$trimmed"
      fi
    fi

    if [[ -f "$dep_path/PKG_CONFIG_PATH" ]]; then
      local data
      data=$(cat "${dep_path}"/PKG_CONFIG_PATH)
      local trimmed
      trimmed=$(trim "$data")
      if [[ -n "$PKG_CONFIG_PATH" ]]; then
        export PKG_CONFIG_PATH="${PKG_CONFIG_PATH}:${trimmed}"
      else
        export PKG_CONFIG_PATH="$trimmed"
      fi
    fi
  done

  # Create a cache directory if it doesn't already exist from `do_unpack()`
  mkdir -pv "$CACHE_PATH"

  # Set PREFIX for maximum default software build support
  export PREFIX=$pkg_prefix
  build_line "Setting PREFIX=$PREFIX"
  build_line "Setting LD_RUN_PATH=$LD_RUN_PATH"
  build_line "Setting CFLAGS=$CFLAGS"
  build_line "Setting CXXFLAGS=$CXXFLAGS"
  build_line "Setting CPPFLAGS=$CPPFLAGS"
  build_line "Setting LDFLAGS=$LDFLAGS"
  build_line "Setting PKG_CONFIG_PATH=$PKG_CONFIG_PATH"
  return 0
}

# **Internal** Modifies any libtool scripts (`ltmain.sh`) in the extracted
# source to remove the default system search path of `/usr/lib`, etc. when
# looking for shared libraries.
_fix_libtool() {
  find "$SRC_PATH" -iname "ltmain.sh" | while read -r file; do
    build_line "Fixing libtool script $file"
    sed -i -e 's^eval sys_lib_.*search_path=.*^^' "$file"
  done
}

# **Internal** Verifies that any lazily-computed, required variables have been
# set, otherwise it fails the build.
_verify_vars() {
  if [[ "${pkg_version:-}" == "__pkg__version__unset__" ]]; then
    local e
    e="Plan did not set 'pkg_version' and did not call 'update_pkg_version()'"
    e="$e before the 'do_prepare()' build phase."
    exit_with "$e" 2
  fi

  _verify_vars=true
}

# **Internal** Verifies that hook files do not contain invalid (CR+LF) line endings
# and fails the build if any are found.
_verify_hook_line_endings() {
  local files
  if files=$(grep --files-with-matches $'\r' "$PLAN_CONTEXT"/hooks/* 2>/dev/null); then
    local e="Incorrect CR+LF line ending detected in package hook file(s)."
    e="$e For the Plan to build, you must first convert these to Unix LF."
    e="$e See https://en.wikipedia.org/wiki/Newline#Conversion_between_newline_formats"
    e=$(printf "%s\n%s" "$e" "$files")
    exit_with "$e" 1
  fi
}

# This function simply makes sure that the working directory for the prepare
# step is correct, that is inside the extracted source directory.
do_prepare_wrapper() {
  build_line "Preparing to build"
  pushd "$SRC_PATH" > /dev/null
  do_prepare
  popd > /dev/null
}

# A step that exists to be overridden. We have the software downloaded,
# unpacked, and the build environment variables set. Do what you need to do
# before we actually run the build steps. Delegates most of the implementation
# to the `do_default_prepare()` function.
do_prepare() {
  do_default_prepare
  return $?
}

# Default implementation of the `do_prepare()` phase.
do_default_prepare() {
  return 0
}

# Since `build` is one of the most overridden functions, this wrapper makes sure
# that no matter how it is changed, our `$cwd` is `$SRC_PATH`.
do_build_wrapper() {
  build_line "Building"
  pushd "$SRC_PATH" > /dev/null
  do_build
  popd > /dev/null
}

# Build the software; assumes the GNU pattern. Delegates most of the
# implementation to the `do_default_build()` function.
do_build() {
  do_default_build
  return $?
}

# Default implementation for the `do_build()` phase.
do_default_build() {
  ./configure --prefix="$pkg_prefix"
  make
}

# Will run post-compile tests and checks, provided 2 conditions are true:
#
# 1. A `do_check()` function has been declared. By default, no such function
#    exists, so Plan author must add one explicitly--there is no reasonably
#    good default here.
# 1. A `$DO_CHECK` environment variable is set to some non-empty value. As
#    tests can dramatically inflate the build time of a Plan, this has been
#    left as an opt-in option.
#
# Here's an example example of a vanilla Plan such as Sed:
#
# ```sh
# pkg_name=sed
# # other Plan metadata...
#
# do_check() {
#   make check
# }
# ```
do_check_wrapper() {
  if [[ "$(type -t do_check)" = "function" && -n "$DO_CHECK" ]]; then
    build_line "Running post-compile tests"
    pushd "$SRC_PATH" > /dev/null
    do_check
    popd > /dev/null
  fi
}

# Identical to the `build_wrapper` function above; simply makes sure the
# working directory for the install_files step is correct.
do_install_wrapper() {
  build_line "Installing"
  mkdir -pv "$pkg_prefix"
  for dir in "${pkg_lib_dirs[@]}"; do
    mkdir -pv "$pkg_prefix/$dir"
  done
  for dir in "${pkg_bin_dirs[@]}"; do
    mkdir -pv "$pkg_prefix/$dir"
  done
  for dir in "${pkg_include_dirs[@]}"; do
    mkdir -pv "$pkg_prefix/$dir"
  done
  for dir in "${pkg_pconfig_dirs[@]}"; do
    mkdir -pv "$pkg_prefix/$dir"
  done
  pushd "$SRC_PATH" > /dev/null
  do_install
  popd > /dev/null
}

# Install the software. Delegates most of the implementation to the
# `do_default_install()` function.
do_install() {
  do_default_install
  return $?
}

# Default implementation for the `do_install()` phase.
do_default_install() {
  make install
}

# **Internal** Write out the package data to files:
#
# * `$pkg_prefix/BUILD_DEPS` - Any dependencies we need build the package
# * `$pkg_prefix/CFLAGS` - Any CFLAGS for things that link against us
# * `$pkg_prefix/PKG_CONFIG_PATH` - Any PKG_CONFIG_PATH entries for things that depend on us
# * `$pkg_prefix/DEPS` - Any dependencies we need to use the package at runtime
# * `$pkg_prefix/EXPORTS` - A list of exported configuration keys and their public name
# * `$pkg_prefix/EXPOSES` - An array of `pkg_exports` for which ports that this package exposes
# * `$pkg_prefix/BINDS` - A list of services you connect to and keys that you expect to be exported
# * `$pkg_prefix/BINDS_OPTIONAL` - Same as `BINDS` but not required for the service to start
# * `$pkg_prefix/FILES` - blake2b checksums of all files in the package
# * `$pkg_prefix/LDFLAGS` - Any LDFLAGS for things that link against us
# * `$pkg_prefix/LD_RUN_PATH` - The LD_RUN_PATH for things that link against us
# * `$pkg_prefix/PATH` - Any PATH entries for things that link against us
_build_metadata() {
  build_line "Building package metadata"

  _render_metadata_LD_RUN_PATH
  _render_metadata_LDFLAGS
  _render_metadata_CFLAGS
  _render_metadata_CPPFLAGS
  _render_metadata_CXXFLAGS
  _render_metadata_PKG_CONFIG_PATH

  _render_metadata_BUILDTIME_ENVIRONMENT
  _render_metadata_BUILDTIME_ENVIRONMENT_PROVENANCE
  _render_metadata_PATH
  _render_metadata_EXPORTS
  _render_metadata_BINDS
  _render_metadata_BINDS_OPTIONAL
  _render_metadata_EXPOSES
  _render_metadata_INTERPRETERS
  _render_metadata_BUILD_DEPS
  _render_metadata_BUILD_TDEPS
  _render_metadata_DEPS
  _render_metadata_TDEPS
  _render_metadata_TARGET
  _render_metadata_IDENT
  _render_metadata_RUNTIME_PATH
  _render_metadata_RUNTIME_ENVIRONMENT
  _render_metadata_RUNTIME_ENVIRONMENT_PROVENANCE

  # Only generate `SVC_USER` & `SVC_GROUP` files if this package is a service.
  # We determine this by checking if there is a `hooks/run` script and/or
  # a set `$pkg_svc_run` value.
  if [[ -f "$PLAN_CONTEXT/hooks/run" || -n "${pkg_svc_run:-}" ]]; then
    _render_metadata_SVC_USER
    _render_metadata_SVC_GROUP
  fi

  return 0
}

# Copy the `./config` directory, relative to the Plan, to `$pkg_prefix/config`.
# Do the same with `default.toml`. Delegates most of the implementation to the
# `do_default_build_config()` function.
do_build_config() {
  do_default_build_config
  return $?
}

# Default implementation for the `do_build_config()` phase.
do_default_build_config() {
  build_line "Writing configuration"
  _do_copy_templates "config"
  if [[ -n "${HAB_FEAT_INSTALL_HOOK:-}" ]]; then
    _do_copy_templates "config_install"
  fi
  if [[ -d "$PLAN_CONTEXT/hooks" ]]; then
    cp -r "$PLAN_CONTEXT/hooks" "$pkg_prefix"
    chmod 755 "$pkg_prefix"/hooks
  fi
  if [[ -f "$PLAN_CONTEXT/default.toml" ]]; then
    cp "$PLAN_CONTEXT/default.toml" "$pkg_prefix"
  fi
  return 0
}

_do_copy_templates() {
  if [[ -d "$PLAN_CONTEXT/$1" ]]; then
    if [[ -z "${HAB_CONFIG_EXCLUDE:-}" ]]; then
      # HAB_CONFIG_EXCLUDE not set, use defaults
      config_exclude_exts=("*.sw?" "*~" "*.bak")
    else
      IFS=',' read -r -a config_exclude_exts <<< "$HAB_CONFIG_EXCLUDE"
    fi
    find_exclusions=()
    for ext in "${config_exclude_exts[@]}"; do
      find_exclusions+=(! -name "$ext")
    done
    find "$PLAN_CONTEXT/$1" "${find_exclusions[@]}" | while read -r FILE
    do
      local plan_context_relative_path="$pkg_prefix${FILE#$PLAN_CONTEXT}"
      if [[ -d "$FILE" ]]; then
        mkdir -p "$plan_context_relative_path"
      else
        cp "$FILE" "$plan_context_relative_path"
      fi
    done
    chmod 755 "$pkg_prefix/$1"
  fi
}

# Write out the `$pkg_prefix/run` file. If a file named `hooks/run`
# exists, we skip this step. Otherwise, we look for `$pkg_svc_run`,
# and use that. We assume that the binary used in the `$pkg_svc_run`
# command is set in the $PATH.
#
# This will write a `run` script that uses `chpst` to run the command
# as the `$pkg_svc_user` and `$pkg_svc_group`. These are `hab` by
# default.
#
# Delegates most of the implementation to the `do_default_build_server()`
# function.
do_build_service() {
  do_default_build_service
  return $?
}

# Default implementation of the `do_build_service()` phase.
do_default_build_service() {
  build_line "Writing service management scripts"
  if [[ -f "${PLAN_CONTEXT}/hooks/run" ]]; then
    build_line "Using run hook ${PLAN_CONTEXT}/hooks/run"
    return 0
  else
    if [[ -n "${pkg_svc_run}" ]]; then
      # We use chpst to ensure that the script works outside `hab-sup`
      # for debugging purposes
      build_line "Writing ${pkg_prefix}/run script to run ${pkg_svc_run} as ${pkg_svc_user}:${pkg_svc_group}"
      cat <<EOT >> "$pkg_prefix"/run
#!/bin/sh
export HOME=$pkg_svc_data_path
cd $pkg_svc_path

if [ "\$(whoami)" = "root" ]; then
  exec chpst \\
    -U ${pkg_svc_user}:${pkg_svc_group} \\
    -u ${pkg_svc_user}:${pkg_svc_group} \\
    ${pkg_svc_run} 2>&1
else
  exec ${pkg_svc_run} 2>&1
fi
EOT
    fi
  fi
  return 0
}

# Strip any binaries, decreasing our total size. Delegates most of the
# implementation to the `do_default_strip()` function.
do_strip() {
  do_default_strip
  return $?
}

# Default implementation for the `do_strip()` phase.
# TODO(SM): Previous versions of the `file` utility reported x-pie-exectuable
# as x-sharedlib. This means that while the intent was to `--strip-all` for
# x-executable, in reality we have been running `--strip-unneeded`. In order to
# be consistant with past behavior we will pass `--strip-unneeded` when stripping
# x-pie-executable. In the future, we will need to make a decision as to the behavior
# we want and introduce it at an appropriate time, such as a core-plans refresh.
# https://gcc.gnu.org/bugzilla/show_bug.cgi?id=81498
# https://bugs.launchpad.net/ubuntu/+source/file/+bug/1747711
do_default_strip() {
  build_line "Stripping unneeded symbols from binaries and libraries"
  find "$pkg_prefix" -type f -perm -u+w -print0 2> /dev/null \
    | while read -rd '' f; do
      case "$(file -bi "$f")" in
        *application/x-executable*) strip --strip-all "$f";;
        *application/x-pie-executable*) strip --strip-unneeded "$f";;
        *application/x-sharedlib*) strip --strip-unneeded "$f";;
        *application/x-archive*) strip --strip-debug "$f";;
        *) continue;;
      esac
    done
}

# At this phase of the build, the package has been built, installed, and
# stripped, but before the package metadata is written and the artifact is
# created and signed.
do_after() {
  do_default_after
  return $?
}

# Default implementation for the `do_after()` phase.
do_default_after() {
  return 0
}

# **Internal** Write the `$pkg_prefix/MANIFEST`.
_build_manifest() {
  build_line "Creating manifest"
  # shellcheck disable=2154
  if [[ -z $pkg_upstream_url ]]; then
    local _upstream_url_string="upstream project's website or home page is not defined"
  else
    local _upstream_url_string="[$pkg_upstream_url]($pkg_upstream_url)"
  fi

  if [[ -z $pkg_source ]]; then
    local _source_url_string="source URL not provided or required"
  else
    local _source_url_string="[$pkg_source]($pkg_source)"
  fi

  if [[ -z $pkg_shasum ]]; then
    local _sha_string="SHA256 checksum not provided or required"
  else
    local _sha_string="\`$pkg_shasum\`"
  fi

  local _build_deps_string
  if [[ -z "${pkg_build_deps[*]}" ]]; then
    _build_deps_string="no build dependencies or undefined"
  else
    _build_deps_string="\`$(printf "%s " "${pkg_build_deps[@]}")\`"
  fi

  local _deps_string
  if [[ -z "${pkg_deps[*]}" ]]; then
    _deps_string="no runtime dependencies or undefined"
  else
    _deps_string="\`$(printf "%s " "${pkg_deps[@]}")\`"
  fi

  local _interpreters_string
  # shellcheck disable=2154
  if [[ -z "${pkg_interpreters[*]}" ]]; then
    _interpreters_string="no interpreters or undefined"
  else
    _interpreters_string="\`$(printf "%s " "${pkg_interpreters[@]}")\`"
  fi

  if [[ -z "$CFLAGS" ]]; then
    local _cflags_string="no CFLAGS"
  else
    local _cflags_string="$CFLAGS"
  fi

  if [[ -z "$CPPFLAGS" ]]; then
    local _cppflags_string="no CPPFLAGS"
  else
    local _cppflags_string="$CPPFLAGS"
  fi

  if [[ -z "$CXXFLAGS" ]]; then
    local _cxxflags_string="no CXXFLAGS"
  else
    local _cxxflags_string="$CXXFLAGS"
  fi

  if [[ -z "$LDFLAGS" ]]; then
    local _ldflags_string="no LDFLAGS"
  else
    local _ldflags_string="$LDFLAGS"
  fi

  if [[ -z "$LD_RUN_PATH" ]]; then
    local _ldrunpath_string="no LD_RUN_PATH"
  else
    local _ldrunpath_string="$LD_RUN_PATH"
  fi

  # shellcheck disable=2154
  cat <<-EOT >> "$pkg_prefix"/MANIFEST
# $pkg_origin / $pkg_name
$pkg_description

* __Maintainer__: $pkg_maintainer
* __Version__: $pkg_version
* __Release__: $pkg_release
* __Target__: $pkg_target
* __Upstream URL__: $_upstream_url_string
* __License__: $(printf "%s " "${pkg_license[@]}")
* __Source__: $_source_url_string
* __SHA__: $_sha_string
* __Path__: \`$pkg_prefix\`
* __Build Dependencies__: $_build_deps_string
* __Dependencies__: $_deps_string
* __Interpreters__: $_interpreters_string

# Plan

## Build Flags

\`\`\`bash
CFLAGS: $_cflags_string
CPPFLAGS: $_cppflags_string
CXXFLAGS: $_cxxflags_string
LDFLAGS: $_ldflags_string
LD_RUN_PATH: $_ldrunpath_string
\`\`\`

## Plan Source

\`\`\`bash
$(cat "$PLAN_CONTEXT"/plan.sh)
\`\`\`
EOT
  return 0
}

# **Internal** Create the package artifact with `tar`/`hab pkg sign`
_generate_artifact() {
  build_line "Generating package artifact"
  local tarf
  tarf="$(dirname "$pkg_artifact")/.$(basename "${pkg_artifact/%.${_artifact_ext}/.tar}")"
  local xzf="${tarf}.xz"

  mkdir -pv "$(dirname "$pkg_artifact")"
  rm -fv "$tarf" "$xzf" "$pkg_artifact"
  $_tar_cmd -cf "$tarf" "$pkg_prefix"
  $_xz_cmd --compress -${HAB_HART_COMPRESSION_LEVEL} --threads=0 "$tarf"
  $HAB_BIN pkg sign --origin $pkg_origin "$xzf" "$pkg_artifact"
  rm -f "$tarf" "$xzf"
}

_prepare_build_outputs() {
  local plan_owner
  _pkg_sha256sum=$($_shasum_cmd "$pkg_artifact" | cut -d " " -f 1)
  _pkg_blake2bsum=$($HAB_BIN pkg hash "$pkg_artifact" | cut -d " " -f 1)
  plan_owner="$(stat -c '%u:%g' "$PLAN_CONTEXT/plan.sh")"

  mkdir -pv "$pkg_output_path"
  # Attempt to set user/group ownership to the same as the ownership of the
  # `plan.sh` file. If the `chown` fails, then don't stop the build--this is
  # only best effort.
  chown "$plan_owner" "$pkg_output_path" || true
  cp -v "$pkg_artifact" "$pkg_output_path"/
  chown "$plan_owner" "$pkg_output_path/$(basename "$pkg_artifact")" || true

  # At this point, we know it built successfully, so delete the pre_build file
  pre_build_file="$pkg_output_path/pre_build.env"
  if [ -f "$pre_build_file" ]; then
    rm -f "$pre_build_file"
  fi

  cat <<-EOF > "$pkg_output_path"/last_build.env
pkg_origin=$pkg_origin
pkg_name=$pkg_name
pkg_version=$pkg_version
pkg_release=$pkg_release
pkg_target=$pkg_target
pkg_ident=${pkg_origin}/${pkg_name}/${pkg_version}/${pkg_release}
pkg_artifact=$(basename "$pkg_artifact")
pkg_sha256sum=$_pkg_sha256sum
pkg_blake2bsum=$_pkg_blake2bsum
EOF
  chown "$plan_owner" "$pkg_output_path/last_build.env" || true
}

# A function for cleaning up after yourself. Delegates most of the
# implementation to the `do_default_end()` function.
do_end() {
  do_default_end
  return $?
}

# Default implementation for the `do_end()` phase.
do_default_end() {
  return 0
}

# Internal: Prevent a failure in `do_after_success` from failing the
# entire job. Execution of this hook is best-effort.
do_after_success_wrapper() {
    _do_final_callback_wrapper "do_after_success"
}

# Internal: Prevent a failure in `do_after_success` from failing the
# entire job. Execution of this hook is best-effort.
do_after_failure_wrapper() {
    _do_final_callback_wrapper "do_after_failure"
}

# **Internal** Call the `do_after_success` or `do_after_failure`
# callback functions (if defined), ensuring that errors in *those*
# functions don't affect the broader build. Failure in a success
# callback shouldn't fail the build, and the exit code of either
# shouldn't be able to change the exit code of the build
# itself. Failure in a callback shouldn't trigger the failure
# callback.
_do_final_callback_wrapper() {
    local callback_function=${1}

    if [[ "$(type -t "${callback_function}")" == "function" ]]; then
        build_line "'${callback_function}' callback function is defined; executing..."

        # We don't want errors in callback execution to affect the
        # broader build; execution of these functions is strictly
        # best-effort, and sugar on top of the build.
        #
        # Thus, we need to do a lot of juggling of error handling.
        #
        # Kinda gross, but it's Bash.

        # Set this to prevent an error from the hook from triggering
        # the error handler.
        set +e
        # Don't want an error from the success hook to trigger the error hook
        trap - ERR
        (
            # We do want to stop early if one of several commands in
            # the hook itself fail.
            set -e

            ${callback_function}
        )
        local retval=$?

        # Set handlers back to what they were for consistency
        trap _on_exit ERR
        set -e

        if [[ $retval -ne 0 ]]; then
            local message="'${callback_function}' callback failed"
            if [[ "${callback_function}" == "do_after_success" ]];
            then
                message="${message}; overall build is successful, though"
            fi
            _build_error_message "${message}"
        fi
    fi
}

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

# Expand the context path to an absolute path
PLAN_CONTEXT="$(abspath "$PLAN_CONTEXT")"
# Set the initial source root to be the same as the Plan context directory.
# This assumes that your application source is local and your Plan exists with
# your code.
SRC_PATH="$PLAN_CONTEXT"
# Expand the path of this program to an absolute path
THIS_PROGRAM=$(abspath "$0")

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

# With plan variants, plans can exist in 4 places per $pkg_target relative to
# the `$PLAN_CONTEXT` directory. Only two combinations are allowed:
#
#   `./plan.sh` AND `./$pkg_target/plan.sh`
#   OR
#   `./habitat/plan.sh` AND `./habitat/$pkg_target/plan.sh`
# Consider all other combination of two plans invalid and abort if found.

# ** Internal ** Relative to the current plan context,  check for a variant
#   that matches the current $pkg_target, and update $PLAN_CONTEXT if found.
_check_for_plan_variant() {
  if [[ -f "$PLAN_CONTEXT/$pkg_target/plan.sh" ]]; then
    PLAN_CONTEXT="$PLAN_CONTEXT/$pkg_target"
  fi
}

# Look for a plan.sh relative to the $PLAN_CONTEXT. If we find an invalid
#   combination or are unable to find a plan.sh,  abort with a message to the
#   user with the failure case.
if [[ -f "$PLAN_CONTEXT/plan.sh" ]]; then
  if [[ -f "$PLAN_CONTEXT/habitat/plan.sh" ]]; then
    places="$PLAN_CONTEXT/plan.sh and $PLAN_CONTEXT/habitat/plan.sh"
    exit_with "A plan file was found at $places. Only one is allowed at a time" 42
  fi
  _check_for_plan_variant
elif [[ -f "$PLAN_CONTEXT/habitat/plan.sh" ]]; then
  PLAN_CONTEXT="$PLAN_CONTEXT/habitat"
  _check_for_plan_variant
else
  places="$PLAN_CONTEXT/plan.sh or $PLAN_CONTEXT/habitat/plan.sh"
  exit_with "Plan file not found at $places" 42
fi

# Change into the `$PLAN_CONTEXT` directory for proper resolution of relative
# paths in `plan.sh`
cd "$PLAN_CONTEXT"

# Load the Plan
build_line "Loading $PLAN_CONTEXT/plan.sh"
if source "$PLAN_CONTEXT/plan.sh"; then
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
for var in "${required_variables[@]}"
do
  if [[ -z "${!var}" ]] ; then
    exit_with "Failed to build. '${var}' must be set." 1
  fi
done

# Test to ensure package name contains only valid characters
for var in pkg_name pkg_origin; do
  if [[ ! "${!var}" =~ ^[A-Za-z0-9_-]+$ ]];
  then
    exit_with "Failed to build. Package $var '${!var}' contains invalid characters." 1
  fi
done

# Pass over `$pkg_svc_run` to replace any `$pkg_name` placeholder tokens
# from prior pkg_svc_* variables that were set before the Plan was loaded.
if [[ -n "${pkg_svc_run+xxx}" ]]; then
    # shellcheck disable=2001
    pkg_svc_run="$(echo $pkg_svc_run | sed "s|@__pkg_name__@|$pkg_name|g")"
fi

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
    pkg_dirname="${pkg_name}-${pkg_version}"
    _pkg_dirname_initially_unset=true
fi

# Set `$pkg_prefix` if not already set by the Plan.
if [[ -z "${pkg_prefix+xxx}" ]]; then
    pkg_prefix=$HAB_PKG_PATH/${pkg_origin}/${pkg_name}/${pkg_version}/${pkg_release}
fi

# Set the cache path to be under the cache source root path
CACHE_PATH="$HAB_CACHE_SRC_PATH/$pkg_dirname"

# If `$pkg_source` is used, update the source path to build under the cache
# source path.
if [[ -n "${pkg_source:-}" ]]; then
    SRC_PATH="$CACHE_PATH"
fi

if [[ -n "$HAB_OUTPUT_PATH" ]]; then
    pkg_output_path="$HAB_OUTPUT_PATH"
else
    pkg_output_path="$INITIAL_PWD/results"
fi

# Set $pkg_svc variables a second time, now that the Plan has been sourced and
# we have access to `$pkg_name`.
pkg_svc_path="$HAB_ROOT_PATH/svc/$pkg_name"
pkg_svc_data_path="$pkg_svc_path/data"
pkg_svc_files_path="$pkg_svc_path/files"
pkg_svc_var_path="$pkg_svc_path/var"
pkg_svc_config_path="$pkg_svc_path/config"
pkg_svc_config_install_path="$pkg_svc_path/config_install"
pkg_svc_static_path="$pkg_svc_path/static"

pkg_artifact="$HAB_CACHE_ARTIFACT_PATH/${pkg_origin}-${pkg_name}-${pkg_version}-${pkg_release}-${pkg_target}.${_artifact_ext}"

# Determine if we have all the commands we need to work
_find_system_commands
_determine_hab_bin

# We removed support for composite plans in habitat 0.75.0.  Let's give a useful error message
# if you happen to try and build one rather than create a useless package
if [[ -n "${pkg_type:-}" && "${pkg_type}" == "composite" ]]; then
     exit_with "Composite plans are no longer supported. For more details see https://www.habitat.sh/blog/2018/10/shelving-composites/" 1
fi

# Run `do_begin`
build_line "$_program setup"
do_begin

# Write out a prebuild file so workers can have some metadata about failed builds
_write_pre_build_file

# Ensure that the origin key is available for package signing
_ensure_origin_key_present

_resolve_dependencies

# Set up runtime and buildtime environments
#
# Alas, this does not actually do *all* the environment setup;
# there are other places which must be accounted for. They
# are:
#
# * before `do_prepare`, but generally in `do_before`
#   This is where it is recommended that authors call
#   `update_plan_version` if they have a dynamic version.
# *_build_environment
#   This is where CFLAGS & Co. are set. At the moment, we
#   don't pull those into __runtime_environment and
#   __buildtime_environment because they are dealt with as
#   their own Special Thing (they've got their own metadata
#   files, etc.)
do_setup_environment_wrapper

# Set up the `PATH` environment variable so that commands will be found
# for all subsequent phases
_set_build_path

mkdir -pv "$HAB_CACHE_SRC_PATH"

# Run any code after the environment is set but before the build starts
do_before

# Download the source
do_download

# Verify the source
do_verify

# Clean the cache
do_clean

# Unpack the source
do_unpack

# Set up the build environment
_build_environment

# Fix any libtool scripts in the source
_fix_libtool

# Make sure all required variables are set
_verify_vars

# Check for invalid (CR+LF) line endings in hooks
_verify_hook_line_endings

# Prepare the source
do_prepare_wrapper

# Build the source
do_build_wrapper

# Check the source
do_check_wrapper

# Install the source
do_install_wrapper

# Copy the configuration
do_build_config

# Copy the service management scripts
do_build_service

# Strip the binaries
do_strip

# Run any code after the package has finished building and installing, but
# before the artifact metadata is generated and the artifact is signed.
do_after

# Render the linking and dependency files
_build_metadata

# The FILES file must be the last metadata file generated, as it lists
# all the other metadata files within it.
_render_metadata_FILES

# Write the manifest
_build_manifest

# Generate the artifact and write to artifact cache
_generate_artifact

# Copy produced artifact to a local relative directory
_prepare_build_outputs

# Cleanup
build_line "$_program cleanup"
do_end

# Fire success callback; if we've failed anywhere else, we will have
# fired our `do_after_failure` callback, if available (and also never
# gotten down to here).
do_after_success_wrapper

# Print the results
build_line
build_line "Source Path: $SRC_PATH"
build_line "Installed Path: $pkg_prefix"
build_line "Artifact: $pkg_output_path/$(basename "$pkg_artifact")"
build_line "Build Report: $pkg_output_path/last_build.env"
build_line "SHA256 Checksum: $_pkg_sha256sum"
build_line "Blake2b Checksum: $_pkg_blake2bsum"

# Exit cleanly
build_line
build_line "I love it when a plan.sh comes together."
build_line
_on_exit 0
