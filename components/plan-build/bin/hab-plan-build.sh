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
# pkg_maintainer="Frank Llyod Wright <flw@example.com>"
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
# 1. Create a libsodium signed tarball of the results
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
# pkg_svc_run="bin/haproxy -f $pkg_svc_config_path/haproxy.conf"
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
# supervisor to load the service. The loaded service will wait to run until it's bind becomes
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
# ## Plan Callbacks
#
# `hab-plan-build` allows you to alter its behavior by defining callbacks
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
# pkg_maintainer="Frank Llyod Wright <flw@example.com>"
# pkg_source=http://www.haproxy.org/download/1.5/src/${pkg_name}-${pkg_version}.tar.gz
# pkg_filename=${pkg_name}-${pkg_version}.tar.gz
# pkg_shasum=6648dd7d6b958d83dd7101eab5792178212a66c884bec0ebcd8abc39df83bb78
# pkg_bin_dirs=(bin)
# pkg_deps=(glibc pcre openssl zlib)
# pkg_svc_run="bin/haproxy -f $pkg_svc_config_path/haproxy.conf"
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
: ${HAB_ROOT_PATH:=/hab}
# The default path where source artifacts are downloaded, extracted, & compiled
HAB_CACHE_SRC_PATH=$HAB_ROOT_PATH/cache/src
# The default download root path for package artifacts, used on package
# installation
HAB_CACHE_ARTIFACT_PATH=$HAB_ROOT_PATH/cache/artifacts
# The default path where cryptographic keys are stored. If the
# `$HAB_CACHE_KEY_PATH` environment variable is set, this value is overridden,
# otherwise it is set to its default.
: ${HAB_CACHE_KEY_PATH:=$HAB_ROOT_PATH/cache/keys}
# Export the key path for other programs and subshells to use
export HAB_CACHE_KEY_PATH
# The root path containing all locally installed packages
HAB_PKG_PATH=$HAB_ROOT_PATH/pkgs
# The first argument to the script is a Plan context directory, containing a
# `plan.sh` file
PLAN_CONTEXT=${1:-.}
# The default Habitat Depot from where to download dependencies. If
# `HAB_DEPOT_URL` is set, this value is overridden.
: ${HAB_DEPOT_URL:=https://willem.habitat.sh/v1/depot}
# Export the Depot URL so all other programs and subshells use this same one
export HAB_DEPOT_URL
# The default Habitat channel from where to download dependencies. If
# `HAB_DEPOT_CHANNEL` is set, this value is overridden.
: ${HAB_DEPOT_CHANNEL:=stable}
# Export the Depot channel so all other programs and subshells use this same one
export HAB_DEPOT_CHANNEL
# Fall back here if package can't be installed from $HAB_DEPOT_CHANNEL
FALLBACK_CHANNEL="stable"
# The value of `$PATH` on initial start of this program
INITIAL_PATH="$PATH"
# The value of `pwd` on initial start of this program
INITIAL_PWD="$(pwd)"
# The target architecture this plan will be built for
pkg_arch=$(uname -m | tr '[:upper:]' '[:lower:]')
# The target system (i.e. operating system variant) this plan will be built for
pkg_sys=$(uname -s | tr '[:upper:]' '[:lower:]')
# The full target tuple this plan will be built for
pkg_target="${pkg_arch}-${pkg_sys}"
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

# The environment variables inside a package
declare -A pkg_env
# The build environment variables inside a package
declare -A pkg_build_env
# The internal field separator used to join `env` variables for cascading
declare -A _env_default_sep=(
  ['CFLAGS']=' '
  ['CPPFLAGS']=' '
  ['CXXFLAGS']=' '
  ['LDFLAGS']=' '
  ['LD_RUN_PATH']=':'
  ['PATH']=':'
  ['PKG_CONFIG_PATH']=':'
)
declare -A pkg_env_sep

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


# ## Private/Internal helper functions
#
# These functions are part of the private/internal API of this program and
# should **not** be used externally by Plan authors or other programs. Their
# behaviors and names can change with little to no warning and no direct
# support can be provided as a result. Thank you for your
# understanding--maintaining a tiny but robust public interface is not an easy
# task.

# **Internal** Handles exiting the program on signals. Takes either an argument
# with the status code, or uses the last commands status code.
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
  local exit_status=${1:-$?}
  if [[ $BASH_SUBSHELL -gt 0 ]]; then
    exit $exit_status
  fi
  : ${pkg_name:=unknown}
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
    if [[ "${HAB_NOCOLORING:-}" == "true" ]]; then
      echo "   ${pkg_name}: Exiting on error"
    else
      case "${TERM:-}" in
        *term | xterm-* | rxvt | screen | screen-*)
          echo -e "   \033[1;36m${pkg_name}: \033[1;31mExiting on error\033[0m"
          ;;
        *)
          echo "   ${pkg_name}: Exiting on error"
          ;;
      esac
    fi
  fi
  exit $exit_status
}

# Call the `_on_exit()` function above on:
# * HUP (1)
# * INT (2)
# * QUIT (3)
# * TERM (15)
# * ERR - when a shell command raises an error. Useful for `set -e; set -E`
#   above.
trap _on_exit 1 2 3 15 ERR

_ensure_origin_key_present() {
  local cache="$HAB_CACHE_KEY_PATH"
  local keys_found="$(find $cache -name "${pkg_origin}-*.sig.key" | wc -l)"
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
# * `$_sort_cmd` (GNU version from coreutils)
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
  if $(sort --version 2>&1 | grep -q 'GNU coreutils'); then
    _sort_cmd=$(command -v sort)
  else
    if $(/usr/bin/sort --version 2>&1 | grep -q 'GNU coreutils'); then
      _sort_cmd=/usr/bin/sort
    else
      exit_with "We require GNU sort to find the latest package; aborting" 1
    fi
  fi
  debug "Setting _sort_cmd=$_sort_cmd"

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

  if $(tar --version 2>&1 | grep -q 'GNU tar'); then
    _tar_cmd=$(command -v tar)
  else
    if $(/bin/tar --version 2>&1 | grep -q 'GNU tar'); then
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
  debug "Setting _hab_cmd=$_hab_cmd"

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
  if [[ ! -d "$HAB_PKG_PATH/$1" ]]; then
    warn "No installed packages of '$1' were found"
    return 1
  fi

  # Count the number of slashes, and use it to make a choice
  # about what to return as the latest package.
  local latest_package_flags=$(echo $1 | grep -o '/' | wc -l)
  local depth
  local result
  case $(trim $latest_package_flags) in
    3) depth=1 ;;
    2) depth=2 ;;
    1) depth=3 ;;
  esac
  result=$(find $HAB_PKG_PATH/${1} -maxdepth $depth -type f -name MANIFEST \
    | $_sort_cmd --version-sort -r | head -n 1)
  if [[ -z "$result" ]]; then
    warn "Could not find a suitable installed package for '$1'"
    return 1
  else
    echo "$(dirname $result)"
    return 0
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
  if [[ -z "${NO_INSTALL_DEPS:-}" ]]; then
    $HAB_BIN install -u $HAB_DEPOT_URL --channel $HAB_DEPOT_CHANNEL "$dep" || {
      if [[ "$HAB_DEPOT_CHANNEL" != "$FALLBACK_CHANNEL" ]]; then
        build_line "Trying to install '$dep' from '$FALLBACK_CHANNEL'"
        $HAB_BIN install -u $HAB_DEPOT_URL --channel "$FALLBACK_CHANNEL" "$dep" || true
      fi
    }
  fi
  return 0
}

# **Internal** Returns (on stdout) the `TDEPS` file contents of another locally
# installed package which contain the set of all direct and transitive run
# dependencies. An empty set could be returned as whitespace and/or newlines.
# The lack of a `TDEPS` file in the desired package will be considered an
# unset, or empty set.
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
_get_tdeps_for() {
  local pkg_path="$1"
  if [[ -f "$pkg_path/TDEPS" ]]; then
    cat $pkg_path/TDEPS
  else
    # No file, meaning an empty set
    echo
  fi
  return 0
}

# **Internal** Returns (on stdout) the `DEPS` file contents of another locally
# installed package which contain the set of all direct run dependencies. An
# empty set could be returned as whitespace and/or newlines.  The lack of a
# `DEPS` file in the desired package will be considered an unset, or empty set.
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
_get_deps_for() {
  local pkg_path="$1"
  if [[ -f "$pkg_path/DEPS" ]]; then
    cat $pkg_path/DEPS
  else
    # No file, meaning an empty set
    echo
  fi
  return 0
}

# **Internal** Appends an entry to the given array only if the entry is not
# already present and returns the resulting array back on stdout. In so doing,
# this function mimics a set when adding new entries. Note that any array can
# be passed in, including ones that already contain duplicate entries.
#
# ```
# arr=(a b c)
# arr=($(_return_or_append_to_set "b" "${arr[@]}"))
# echo ${arr[@]}
# # a b c
# arr=($(_return_or_append_to_set "z" "${arr[@]}"))
# echo ${arr[@]}
# # a b c z
# ```
#
# Will return 0 in any case.
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
    | sed -e "$((${lnum}-${context})),$((${lnum}+${context}))!d" \
      -e 's,^,    ,g' \
    | sed -e "$((${context}+1))s/^   / =>/"
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

  for dep in "${pkg_scaffolding}"; do
    _install_dependency $dep
    # Add scaffolding package to the list of scaffolding build deps
    scaff_build_deps+=($dep)
    if resolved="$(_resolve_dependency $dep)"; then
      build_line "Resolved scaffolding dependency '$dep' to $resolved"
      scaff_build_deps_resolved+=($resolved)
      # Add each (fully qualified) direct run dependency of the scaffolding
      # package.
      sdeps=($(_get_deps_for "$resolved"))
      for sdep in "${sdeps[@]}"; do
        scaff_build_deps+=($sdep)
        scaff_build_deps_resolved+=($HAB_PKG_PATH/$sdep)
      done
    else
      exit_with "Resolving '$dep' failed, should this be built first?" 1
    fi
  done

  # Add all of the ordered scaffolding dependencies to the start of
  # `${pkg_build_deps[@]}` to make sure they could be overridden by a Plan
  # author if required.
  pkg_build_deps=(${scaff_build_deps[@]} ${pkg_build_deps[@]})
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
    _install_dependency $dep
    if resolved="$(_resolve_dependency $dep)"; then
      build_line "Resolved build dependency '$dep' to $resolved"
      pkg_build_deps_resolved+=($resolved)
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
    tdeps=($(_get_tdeps_for $dep))
    for tdep in "${tdeps[@]}"; do
      tdep="$HAB_PKG_PATH/$tdep"
      pkg_build_tdeps_resolved=(
        $(_return_or_append_to_set "$tdep" "${pkg_build_tdeps_resolved[@]}")
      )
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

  lib="$(_pkg_path_for_build_deps $pkg_scaffolding)/lib/scaffolding.sh"
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
    _install_dependency $dep
    if resolved="$(_resolve_dependency $dep)"; then
      build_line "Resolved dependency '$dep' to $resolved"
      pkg_deps_resolved+=($resolved)
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
    tdeps=($(_get_tdeps_for $dep))
    for tdep in "${tdeps[@]}"; do
      tdep="$HAB_PKG_PATH/$tdep"
      pkg_tdeps_resolved=(
        $(_return_or_append_to_set "$tdep" "${pkg_tdeps_resolved[@]}")
      )
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
    pkg_all_tdeps_resolved=(
      $(_return_or_append_to_set "$dep" "${pkg_all_tdeps_resolved[@]}")
    )
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
  local tdeps=$(echo ${pkg_tdeps_resolved[@]} \
    | tr ' ' '\n' \
    | sed "s,^${HAB_PKG_PATH}/,,")
  # Build the list of any runtime deps that appear more than once. That is,
  # `ORIGIN/NAME` token duplicates.
  local dupes=$(echo "$tdeps" \
    | awk -F/ '{print $1"/"$2}' \
    | sort \
    | uniq -d)

  if [[ -n "$dupes" ]]; then
    local dupe
    # Build a list of all fully qualified package identifiers that are members
    # of the duplicated `ORIGIN/NAME` tokens. This will be used to star the
    # problematic dependencies in the graph.
    _dupes_qualified=$(echo "$tdeps" \
      | egrep "($(echo "$dupes" | tr '\n' '|' | sed 's,|$,,'))")

    warn
    warn "The following runtime dependencies have more than one version"
    warn "release in the full dependency chain:"
    warn
    echo "$dupes" | while read dupe; do
      warn "  * $dupe ( $(echo "$tdeps" | grep "$dupe" | tr '\n' ' '))"
    done
    warn
    warn 'The current situation usually arises when a Plan has a direct '
    warn 'dependency on one version of a package (`acme/A/7.0/20160101200001`)'
    warn 'and has a direct dependency on another package which itself depends'
    warn 'on another version of the same package (`acme/A/2.0/20151201060001`).'
    warn 'If this package (`acme/A`) contains shared libraries which are'
    warn 'loaded at runtime by the current Plan, then both versions of'
    warn '`acme/A` could be loaded into the same process in a potentially'
    warn 'surprising order. Worse, if both versions of `acme/A` are'
    warn 'ABI-incompatible, runtime segmentation faults are more than likely.'
    warn
    warn 'In order to preserve reliability at runtime the duplicate dependency'
    warn 'entries will need to be resolved before this Plan can be built.'
    warn 'Below is an expanded graph of all `$pkg_deps` and their dependencies'
    warn 'with the problematic lines noted.'
    warn
    warn "Computed dependency graph (Lines with '*' denote a problematic entry):"
    printf "\n${pkg_origin}/${pkg_name}/${pkg_version}/${pkg_release}\n"
    echo ${pkg_deps_resolved[@]} \
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
  local padn=$(($level * 4))
  while read dep; do
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
      cat $HAB_PKG_PATH/$dep/DEPS | _print_recursive_deps $(($level + 1))
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
    if echo $e | cut -d "/" -f ${cutn}- | egrep -q "(^|/)${dep}(/|$)"; then
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
    if echo $e | cut -d "/" -f ${cutn}- | egrep -q "(^|/)${dep}(/|$)"; then
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


# Check that the command exists, 0 if it does, 1 if it does not.
#
# ```sh
# exists gsha256sum
# ```
#
# Would return 0 if gsha256sum exists, 1 if it does not.
exists() {
  if command -v $1 >/dev/null 2>&1
  then
    return 0
  else
    return 1
  fi
}

# Print a line of build output. Takes the rest of the line as its only
# argument.
#
# ```sh
# build_line "Checksum verified - ${pkg_shasum}"
# ```
build_line() {
  if [[ "${HAB_NOCOLORING:-}" == "true" ]]; then
    echo "   ${pkg_name}: $1"
  else
    case "${TERM:-}" in
      *term | xterm-* | rxvt | screen | screen-*)
        echo -e "   \033[1;36m${pkg_name}: \033[1;37m$1\033[0m"
        ;;
      *)
        echo "   ${pkg_name}: $1"
        ;;
    esac
  fi
  return 0
}

# Print a warning line on stderr. Takes the rest of the line as its only
# argument.
#
# ```sh
# warn "Checksum failed"
# ```
warn() {
  if [[ "${HAB_NOCOLORING:-}" == "true" ]]; then
    >&2 echo "   ${pkg_name}: WARN $1"
  else
    case "${TERM:-}" in
      *term | xterm-* | rxvt | screen | screen-*)
        >&2 echo -e "   \033[1;36m${pkg_name}: \033[1;33mWARN \033[1;37m$1\033[0m"
        ;;
      *)
        >&2 echo "   ${pkg_name}: WARN $1"
        ;;
    esac
  fi
  return 0
}

# Prints a line only if the `$DEBUG` environment value is set.
#
# ```sh
# DEBUG=1
# debug "Only if things are set"
# # "DEBUG: Only if things are set"
# DEBUG=0
# debug "Not so much anymore"
# ```
#
debug() {
  if [[ -n "$DEBUG" ]]; then
    echo "DEBUG: $1"
  fi
  return 0
}

# Exit the program with an error message and a status code.
#
# ```
# exit_with "Something bad went down" 55
# ```
exit_with() {
  if [[ "${HAB_NOCOLORING:-}" == "true" ]]; then
    echo "ERROR: $1"
  else
    case "${TERM:-}" in
      *term | xterm-* | rxvt | screen | screen-*)
        echo -e "\033[1;31mERROR: \033[1;37m$1\033[0m"
        ;;
      *)
        echo "ERROR: $1"
        ;;
    esac
  fi
  exit $2
}

# Trim leading and trailing whitespace.  [Thanks to these
# guys](http://stackoverflow.com/questions/369758/how-to-trim-whitespace-from-bash-variable)
# for the tip.
#
# ```sh
# local data=$(cat /tmp/somefile)
# local trimmed=$(trim $data)
# ```
trim() {
  local var="$*"
  var="${var#"${var%%[![:space:]]*}"}"   # remove leading whitespace characters
  var="${var%"${var##*[![:space:]]}"}"   # remove trailing whitespace characters
  echo "$var"
}

# Returns the path for the desired build or runtime direct package dependency
# on stdout from the resolved dependency set.
#
# ```
# pkg_all_deps_resolved=(
#   /hab/pkgs/acme/zlib/1.2.8/20151216221001
#   /hab/pkgs/acme/nginx/1.8.0/20150911120000
#   /hab/pkgs/acme/glibc/2.22/20151216221001
# )
#
# pkg_path_for acme/nginx
# # /hab/pkgs/acme/nginx/1.8.0/20150911120000
# pkg_path_for zlib
# # /hab/pkgs/acme/zlib/1.2.8/20151216221001
# pkg_path_for glibc/2.22
# # /hab/pkgs/acme/glibc/2.22/20151216221001
# ```
#
# Will return 0 if a package is found locally on disk, and 1 if a package
# cannot be found. A message will be printed to stderr to provide context.
pkg_path_for() {
  local dep="$1"
  local e
  local cutn="$(($(echo $HAB_PKG_PATH | grep -o '/' | wc -l)+2))"
  for e in "${pkg_all_deps_resolved[@]}"; do
    if echo $e | cut -d "/" -f ${cutn}- | egrep -q "(^|/)${dep}(/|$)"; then
      echo "$e"
      return 0
    fi
  done
  warn "pkg_path_for() '$dep' did not find a suitable installed package"
  warn "Resolved package set: (${pkg_all_deps_resolved[*]})"
  return 1
}

# Attach to an interactive debugging session which lets the user check the
# state of variables, call arbitrary functions, turn on higher levels of
# logging (with `set -x`), or whatever else is useful.
#
# Usage: simply add `attach` in a `plan.sh` file and a debugging session will
# spawn, similar to:
#
# ```
# ### Attaching to debugging session
#
# From: /plans/glibc/plan.sh @ line 66 :
#
#     56:
#     57:   # Modify the ldd rewrite script to remove lib64 and libx32
#     58:   sed -i '/RTLDLIST/d' sysdeps/unix/sysv/linux/*/ldd-rewrite.sed
#     59:
#     60:   rm -rf ../${pkg_name}-build
#     61:   mkdir ../${pkg_name}-build
#     62:   pushd ../${pkg_name}-build > /dev/null
#     63:     # Configure Glibc to install its libraries into `$pkg_prefix/lib`
#     64:     echo "libc_cv_slibdir=$pkg_prefix/lib" >> config.cache
#     65:
#  => 66:     attach
#     67:
#     68:     ../$pkg_dirname/configure \
#     69:       --prefix=$pkg_prefix \
#     70:       --libdir=$pkg_prefix/lib \
#     71:       --libexecdir=$pkg_prefix/lib/glibc \
#     72:       --enable-obsolete-rpc \
#     73:       --disable-profile \
#     74:       --enable-kernel=2.6.32 \
#     75:       --cache-file=config.cache
#     76:     make
#
# [1] glibc(build)>
# ```
attach() {
  printf "\n### Attaching to debugging session\n"
  local cmd=""
  local fname="${FUNCNAME[1]}"
  local replno=1
  # Print out our current code context (source file, line number, etc.)
  _attach_whereami
  # Clear on exit traps and allow for non-zero returning commands as we're
  # entering a debugging session, remember?
  trap - 1 2 3 15 ERR
  set +e
  # Loop through input, REPL-style until either `"exit"` or `"quit"` is found
  while [[ "$cmd" != "exit" && "$cmd" != "quit" ]]; do
    read -e -r -p "[$replno] ${pkg_name}($fname)> " cmd
    history -s $cmd
    case "$cmd" in
      vars) (set -o posix; set);;
      whereami*|\@*)
        _attach_whereami "$(echo $cmd \
         | awk '{if (NF == 2) print $2; else print "10"}')"
        ;;
      exit|quit) ;;
      exit-program|quit-program) exit $?;;
      help)
        printf "
Help
  help          Show a list of command or information about a specific command.

Context
  whereami      Show the code surrounding the current context
                (add a number to increase the lines of context).

Environment
  vars          Prints all the environment variables that are currently in scope.

Navigating
  exit          Pop to the previous context.
  exit-program  End the $0 program.

Aliases
  @             Alias for \`whereami\`.
  quit          Alias for \`exit\`.
  quit-program  Alias for \`exit-program\`.

"
        ;;
      *) eval $cmd;;
    esac
    # Increment our REPL command line count, cause that's helpful
    replno=$((${replno}+1))
  done
  # Re-enable on exit trap and restore exit-on-non-zero behavior
  trap _on_exit 1 2 3 15 ERR
  set -e
  printf "\n### Leaving debugging session\n\n"
  return 0
}

# Return the absolute path for a path, which might be absolute or relative.
#
# ```sh
# abspath .
# # /a/b/c
# abspath /tmp/
# # /tmp
# ```
#
# Thanks to [Stack
# Overflow](http://stackoverflow.com/questions/7126580/expand-a-possible-relative-path-in-bash#answer-13087801)
abspath() {
  if [[ -d "$1" ]]; then
    pushd "$1" > /dev/null
    pwd
    popd >/dev/null
  elif [[ -e $1 ]]; then
    pushd "$(dirname "$1")" > /dev/null
    echo "$(pwd)/$(basename "$1")"
    popd >/dev/null
  else
    echo "$1" does not exist! >&2
    return 127
  fi
}

# Returns all items joined by defined IFS
#
# ```sh
# join_by , a "b c" d
# # a,b c,d
# join_by / var local tmp
# # var/local/tmp
# join_by , "${FOO[@]}"
# # a,b,c
# ```
#
# Thanks to [Stack Overflow](http://stackoverflow.com/a/17841619/515789)
join_by() {
  local IFS="$1"
  shift
  echo "$*"
}

# Sets environment variable for package
#
# ```sh
# add_env PATH 'bin' 'sbin'
# add_env SETTINGS_MODULE 'app.settings'
# ```
add_env() {
  local -u key=$1
  shift
  local values=($*)

  if [[ ${pkg_env[$key]+abc} ]]; then
    exit_with "Cannot add $key to pkg_env once the value is already set"
  fi

  if [[ -n ${values} ]]; then
    # Set a default separator if none is defined
    if [[ ! ${pkg_env_sep[$key]+abc} && ${_env_default_sep[$key]+abc} ]]; then
      pkg_env_sep[$key]=${_env_default_sep[$key]}
    fi

    if [[ ${#values[@]} -gt 1 ]]; then
      if [[ ${pkg_env_sep[$key]+abc} ]]; then
        pkg_env[$key]=$(join_by ${pkg_env_sep[$key]} ${values[@]})
      else
        exit_with "Cannot add multiple values without setting a separator for $key"
      fi
    else
      pkg_env[$key]=${values[0]}
    fi
  fi
}

# Adds `$pkg_prefix` to supplied paths
#
# ```sh
# add_path_env PATH 'bin' 'sbin'
# ```
add_path_env() {
  local key=$1
  shift
  local paths=()
  for path in $*; do
    paths+=("${pkg_prefix}/${path}")
  done
  add_env ${key} ${paths[@]}
}

# TODO: Make `add_build_env` and `add_build_path_env` more generic
# Sets build environment variable for package
#
# ```sh
# add_build_env PATH 'bin' 'sbin'
# add_build_env SETTINGS_MODULE 'app.settings'
# ```
add_build_env() {
  local -u key=$1
  shift
  local values=($*)

  if [[ ${pkg_build_env[$key]+abc} ]]; then
    exit_with "Cannot add $key to pkg_build_env once the value is already set"
  fi

  if [[ -n ${values} ]]; then
    # Set a default separator if none is defined
    if [[ ! ${pkg_env_sep[$key]+abc} && ${_env_default_sep[$key]+abc} ]]; then
      pkg_env_sep[$key]=${_env_default_sep[$key]}
    fi

    if [[ ${#values[@]} -gt 1 ]]; then
      if [[ ${pkg_env_sep[$key]+abc} ]]; then
        pkg_build_env[$key]=$(join_by ${pkg_env_sep[$key]} ${values[@]})
      else
        exit_with "Cannot add multiple values without setting a separator for $key"
      fi
    else
      pkg_build_env[$key]=${values[0]}
    fi
  fi
}

# Adds `$pkg_prefix` to supplied paths
#
# ```sh
# add_build_path_env PATH 'bin' 'sbin'
# ```
add_build_path_env() {
  local key=$1
  shift
  local paths=()
  for path in $*; do
    paths+=("${pkg_prefix}/${path}")
  done
  add_build_env ${key} ${paths[@]}
}

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
    local -i port_num=$(_to_int $port 2>/dev/null)
    if (( $port_num < 1 || $port_num > 65535 )) ; then
        return 1
    fi
    return 0
}

# Downloads a file from a source URL to a local file and uses an optional
# shasum to determine if an existing file can be used.
#
# If an existing file is present and the third argument is set with a shasum
# digest, the file will be checked to see if it's valid. If so, the function
# ends early and returns 0. Otherwise, the shasums do not match so the
# file-on-disk is removed and a normal download proceeds as though no previous
# file existed. This is designed to restart an interrupted download.
#
# Any valid `wget` URL will work.
#
# ```sh
# download_file http://example.com/file.tar.gz file.tar.gz
# # Downloads every time, even if the file exists locally
# download_file http://example.com/file.tar.gz file.tar.gz abc123...
# # Downloads if no local file is found
# download_file http://example.com/file.tar.gz file.tar.gz abc123...
# # File matches checksum: download is skipped, local file is used
# download_file http://example.com/file.tar.gz file.tar.gz oh noes...
# # File doesn't match checksum: local file removed, download attempted
# ```
#
# Will return 0 if a file was downloaded or if a valid cached file was found.
download_file() {
  local url="$1"
  local dst="$2"
  local sha="$3"

  pushd $HAB_CACHE_SRC_PATH > /dev/null
  if [[ -f $dst && -n "$sha" ]]; then
    build_line "Found previous file '$dst', attempting to re-use"
    if verify_file $dst $sha; then
      build_line "Using cached and verified '$dst'"
      return 0
    else
      build_line "Clearing previous '$dst' file and re-attempting download"
      rm -fv $dst
    fi
  fi

  build_line "Downloading '$url' to '$dst'"
  $_wget_cmd $url -O $dst
  build_line "Downloaded '$dst'";
  popd > /dev/null
}

# Verifies that a file on disk matches the given shasum. If the given shasum
# doesn't match the file's shasum then a warning is printed with the expected
# and computed shasum values.
#
# ```sh
# verify_file file.tar.gz abc123...
# ```
#
# Will return 0 if the shasums match, and 1 if they do not match. A message
# will be printed to stderr with the expected and computed shasum values.
verify_file() {
  build_line "Verifying $1"
  local checksum=($($_shasum_cmd $HAB_CACHE_SRC_PATH/$1))
  if [[ $2 = $checksum ]]; then
    build_line "Checksum verified for $1"
  else
    warn "Checksum invalid for $1:"
    warn "   Expected: $2"
    warn "   Computed: ${checksum}"
    return 1
  fi
  return 0
}

# Unpacks an archive file in a variety of formats.
#
# ```sh
# unpack_file file.tar.gz
# ```
#
# Will return 0 if the file archive is extracted, and 1 if the file archive
# type could not be found or was not supported (given the file extension). A
# message will be printed to stderr to provide context.
unpack_file() {
  build_line "Unpacking $1"
  local unpack_file="$HAB_CACHE_SRC_PATH/$1"
  # Thanks to:
  # http://stackoverflow.com/questions/17420994/bash-regex-match-string
  if [[ -f $unpack_file ]]; then
    pushd $HAB_CACHE_SRC_PATH > /dev/null
    case $unpack_file in
      *.tar.bz2|*.tbz2|*.tar.gz|*.tgz|*.tar|*.xz)
        $_tar_cmd xf $unpack_file
        ;;
      *.bz2)  bunzip2 $unpack_file    ;;
      *.rar)  rar x $unpack_file      ;;
      *.gz)   gunzip $unpack_file     ;;
      *.zip)  unzip -o $unpack_file   ;;
      *.Z)    uncompress $unpack_file ;;
      *.7z)   7z x $unpack_file       ;;
      *)
        warn "Error: unknown archive format '.${unpack_file##*.}'"
        return 1
        ;;
    esac
  else
    warn "'$1' is not a valid file!"
    return 1
  fi
  popd > /dev/null
  return 0
}

# Edit the `#!` shebang of the target file in-place. Useful for
# changing hardcoded `/usr/bin/env` to our coreutils, for example. Be
# sure to depend on the required package that provides the expected
# path for the shebang in `pkg_deps`. This does a greedy match against
# the specified interpreter in the target file(s).
#
# To use this function in your plan.sh, specify the following
# arguments:
#
# 1. The target file or files
# 2. The name of the package that contains the interpreter
# 3. The relative directory and binary path to the interpreter
#
# For example, to replace all the files in `node_modules/.bin` that
# have `#!/usr/bin/env` with the `coreutils` path
# to `bin/env` (which resolves to
# /hab/pkgs/acme/coreutils/8.24/20160219013458/bin/env), be sure
# to quote the wildcard target:
#
#     fix_interpreter "node_modules/.bin/*" acme/coreutils bin/env
#
# For a single target:
#
#     fix_interpreter node_modules/.bin/concurrent acme/coreutils bin/env
#
# To get the interpreters exposed by a package, look in that package's
# INTERPRETERS metadata file, e.g.,
# `/hab/pkgs/acme/coreutils/8.24/20160219013458/INTERPRETERS`

fix_interpreter() {
    local targets=$1
    local pkg=$2
    local int=$3
    local interpreter_old=".*${int}"
    local interpreter_new="$(pkg_interpreter_for ${pkg} ${int})"
    if [[ -z $interpreter_new || $? -ne 0 ]]; then
      warn "fix_interpreter() '$pkg' is not a runtime dependency"
      warn "Only runtime packages may be used as your interpreter must travel"
      warn "with the '$pkg_name' in order to run."
      warn "Resolved runtime package set: ${pkg_deps_resolved[*]}"
      return 1
    fi

    for t in ${targets}; do
      if [[ ! -f $t ]]; then
        debug "Ignoring non-file target: ${t}"
        continue
      fi

      # Resolve symbolic links to fix the actual file instead of replacing it
      if [[ -L $t ]]; then
        t="$(readlink --canonicalize --no-newline "$t")"
      fi

      build_line "Replacing '${interpreter_old}' with '${interpreter_new}' in '${t}'"
      sed -e "s#\#\!${interpreter_old}#\#\!${interpreter_new}#" -i $t
    done
}

# Returns the path for the given package and interpreter by reading it
# from the INTERPRETERS metadata in the package. The directory of the
# interpreter needs to be specified, as an interpreter binary might
# live in `bin`, `sbin`, or `libexec`, depending on the software.
#
# ```
# pkg_interpreter_for acme/coreutils bin/env
# ```
#
# Will return 0 if the specified package and interpreter were found,
# and 1 if the package could not be found or the interpreter is not
# specified for that package.
pkg_interpreter_for() {
    local pkg=$1
    local int=$2
    local path=$(_pkg_path_for_deps $pkg)
    if [[ -z $path || $? -ne 0 ]]; then
      warn "Could not resolve the path for ${pkg}, is it specified in 'pkg_deps'?"
      return 1
    fi

   local int_path=$(grep -x ".*${int}" ${path}/INTERPRETERS)
    if [[ -n "$int_path" ]]; then
      echo "$int_path"
      return 0
    fi
    warn "Could not find interpreter ${int} in package ${pkg}"
    return 1
}

# Updates the value of `$pkg_version` and recomputes any relevant variables.
# This function must be called before the `do_prepare()` build phase otherwise
# it will fail the build process.
#
# This function depends on the Plan author implementing a `pkg_version()`
# function which prints a computed version string on standard output. Then,
# this function must be explicitly called in an appropriate build phase--most
# likely `do_before()`. For example:
#
# ```sh
# pkg_origin=acme
# pkg_name=myapp
#
# pkg_version() {
#   cat "$SRC_PATH/version.txt"
# }
#
# do_before() {
#   do_default_before
#   update_pkg_version
# }
# ```
update_pkg_version() {
  local update_src_path val

  if [[ "${_verify_vars:-}" == true ]]; then
    local e
    e="Plan called 'update_pkg_version()' in phase 'do_prepare()' or later"
    e="$e which is not supported. Package version must be determined before"
    e="$e 'do_prepare()' phase."
    exit_with "$e" 21
  fi

  if [[ "$(type -t pkg_version)" == "function" ]]; then
    pkg_version="$(pkg_version)"
    build_line "Version updated to '$pkg_version'"
  else
    debug "pkg_version() function not found, retaining pkg_version=$pkg_version"
  fi

  # `$pkg_dirname` needs to be recomputed, unless it was explicitly set by the
  # Plan author.
  if [[ "${_pkg_dirname_initially_unset:-}" == true ]]; then
    pkg_dirname="${pkg_name}-${pkg_version}"
  fi
  pkg_prefix=$HAB_PKG_PATH/${pkg_origin}/${pkg_name}/${pkg_version}/${pkg_release}
  pkg_artifact="$HAB_CACHE_ARTIFACT_PATH/${pkg_origin}-${pkg_name}-${pkg_version}-${pkg_release}-${pkg_target}.${_artifact_ext}"
  # If the `$CACHE_PATH` and `$SRC_PATH` are the same, then we are building
  # third party software using `$pkg_source` and
  # downloading/verifying/unpacking it.
  if [[ "$CACHE_PATH" == "$SRC_PATH" ]]; then
    update_src_path=true
  fi
  CACHE_PATH="$HAB_CACHE_SRC_PATH/$pkg_dirname"
  # Only update `$SRC_PATH` if we are building third party software using
  # `$pkg_source`.
  if [[ "${update_src_path:-}" == true ]]; then
    SRC_PATH="$CACHE_PATH"
  fi
  # Replace the unset placeholders with the computed value
  val="$(echo "$PATH" | sed "s,__pkg__version__unset__,${pkg_version},g")"
  pkg_env[PATH]="$val"
  PATH="$val"
  build_line "Updating PATH=$PATH"
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
#    build and run dependencies, and the run dependencies for each direct
#    dependency. Further details for this array is described in the
#    `_populate_dependency_arrays()` function.
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

# **Internal**  Build `$PATH` containing each path in our own
# `${pkg_bin_dirs[@]}` array, and then any dependency's `PATH` entry (direct
# or transitive) if one exists. The ordering of the path is specific to
# `${pkg_all_tdeps_resolved[@]}` which is further explained in the
# `_resolve_dependencies()` function.
_set_environment() {
  local -A _environment

  # Set any package pre-build environment variables
  if [[ -n ${pkg_bin_dirs} ]]; then
    add_path_env 'PATH' ${pkg_bin_dirs[@]}
  fi

  # Copy `$pkg_env` to `$_environment`
  for env in "${!pkg_env[@]}"; do
    _environment[$env]=${pkg_env[$env]}
  done

  # Copy `$pkg_build_env` to `$_environment`
  for env in "${!pkg_build_env[@]}"; do
    if [[ ${_environment[$env]+abc} && ${pkg_env_sep[$env]+abc} ]]; then
      _environment[$env]=$(join_by ${pkg_env_sep[$env]} ${_environment[$env]} ${pkg_build_env[$env]})
    elif [[ ! ${_environment[$env]+abc} ]]; then
      _environment[$env]=${pkg_build_env[$env]}
    else
      exit_with "Cannot add $$pkg_build_env without setting a separator for $env"
    fi
  done

  for dep_path in "${pkg_all_tdeps_resolved[@]}"; do
    # If we have a ENVIRONMENT or BUILD_ENVIRONMENT skip looking for legacy files
    if [[ -f "$dep_path/ENVIRONMENT" || -f "$dep_path/BUILD_ENVIRONMENT" ]]; then
      local -A env_sep

      if [[ -f "$dep_path/ENVIRONMENT_SEP" ]]; then
        while read -r line; do
          local -u env=${line%%=*}
          local value=${line#*=}
          if [[ -n "$env" && -n "$value" ]]; then
            env_sep[$env]=${value}
          fi
        done < "$dep_path/ENVIRONMENT_SEP"
      fi

      if [[ -f "$dep_path/ENVIRONMENT" ]]; then
        while read -r line; do
          local -u env=${line%%=*}
          local value=${line#*=}
          if [[ -n "$env" && -n "$value" ]]; then
            if [[ ${_environment[$env]+abc} && ${env_sep[$env]+abc} ]]; then
              _environment[$env]=$(join_by ${env_sep[$env]} ${_environment[$env]} ${value})
            elif [[ ! ${_environment[$env]+abc} ]]; then
              _environment[$env]=${value}
            else
              exit_with "Artifact $dep_path does not have a separator set for $env"
            fi
          fi
        done < "$dep_path/ENVIRONMENT"
      fi

      if [[ -f "$dep_path/BUILD_ENVIRONMENT" ]]; then
        while read -r line; do
          local -u env=${line%%=*}
          local value=${line#*=}
          if [[ -n "$env" && -n "$value" ]]; then
            if [[ ${_environment[$env]+abc} && ${env_sep[$env]+abc} ]]; then
              _environment[$env]=$(join_by ${env_sep[$env]} ${_environment[$env]} ${value})
            elif [[ ! ${_environment[$env]+abc} ]]; then
              _environment[$env]=${value}
            else
              exit_with "Artifact $dep_path does not have a separator set for $env"
            fi
          fi
        done < "$dep_path/BUILD_ENVIRONMENT"
      fi
    else # Look for legacy files
      if [[ -f "$dep_path/PATH" ]]; then
        local data=$(cat "$dep_path/PATH")
        local trimmed=$(trim $data)
        if [[ ${_environment[PATH]+abc} ]]; then
            _environment[PATH]=$(join_by ':' ${_environment[PATH]} ${trimmed})
        else
            _environment[PATH]=${trimmed}
        fi
      fi
    fi
  done
  # Insert all the package PATH fragments before the default PATH to ensure
  # package binaries are used before any userland/operating system binaries
  if [[ ${_environment[PATH]+abc} ]]; then
    _environment[PATH]=$(join_by ':' ${_environment[PATH]} ${INITIAL_PATH})
  fi

  # Export out computed environment
  for env in "${!_environment[@]}"; do
    build_line "Setting $env=${_environment[$env]}"
    export ${env}=${_environment[$env]}
  done
}

# At this phase of the build, all dependencies are downloaded, the build
# environment is set, but this is just before any source downloading would
# occur (if `$pkg_source` is set). This could be a suitable phase in which to
# compute a dynamic version of a pacakge given the state of a Git repository,
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

  download_file $pkg_source $pkg_filename $pkg_shasum
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
    verify_file $pkg_filename $pkg_shasum
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
    unpack_file $pkg_filename
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
      local data=$(cat $dep_path/LD_RUN_PATH)
      local trimmed=$(trim $data)
      ld_run_path_part+=("$trimmed")
    fi
  done
  if [[ -n $ld_run_path_part ]]; then
    export LD_RUN_PATH=$(join_by ':' ${ld_run_path_part[@]})
  fi

  # Build `$CFLAGS`, `$CPPFLAGS`, `$CXXFLAGS` and `$LDFLAGS` containing any
  # direct dependency's `CFLAGS`, `CPPFLAGS`, `CXXFLAGS` or `LDFLAGS` entries
  # respectively (build or run). If the software to be built requires the path
  # to headers or shared libraries, it must be a direct dependency, not
  # transitive.
  for dep_path in "${pkg_all_deps_resolved[@]}"; do
    if [[ -f "$dep_path/CFLAGS" ]]; then
      local data=$(cat $dep_path/CFLAGS)
      local trimmed=$(trim $data)
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
      local data=$(cat $dep_path/CPPFLAGS)
      local trimmed=$(trim $data)
      if [[ -n "$CPPFLAGS" ]]; then
        export CPPFLAGS="$CPPFLAGS $trimmed"
      else
        export CPPFLAGS="$trimmed"
      fi
    fi

    if [[ -f "$dep_path/CXXFLAGS" ]]; then
      local data=$(cat $dep_path/CXXFLAGS)
      local trimmed=$(trim $data)
      if [[ -n "$CXXFLAGS" ]]; then
        export CXXFLAGS="$CXXFLAGS $trimmed"
      else
        export CXXFLAGS="$trimmed"
      fi
    fi

    if [[ -f "$dep_path/LDFLAGS" ]]; then
      local data=$(cat $dep_path/LDFLAGS)
      local trimmed=$(trim $data)
      if [[ -n "$LDFLAGS" ]]; then
        export LDFLAGS="$LDFLAGS $trimmed"
      else
        export LDFLAGS="$trimmed"
      fi
    fi

    if [[ -f "$dep_path/PKG_CONFIG_PATH" ]]; then
      local data=$(cat ${dep_path}/PKG_CONFIG_PATH)
      local trimmed=$(trim $data)
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
  find "$SRC_PATH" -iname "ltmain.sh" | while read file; do
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
  ./configure --prefix=$pkg_prefix
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
# * `$pkg_prefix/BUILD_ENVIRONMENT` - A list of build environment keys and their values
# * `$pkg_prefix/CFLAGS` - Any CFLAGS for things that link against us
# * `$pkg_prefix/PKG_CONFIG_PATH` - Any PKG_CONFIG_PATH entries for things that depend on us
# * `$pkg_prefix/DEPS` - Any dependencies we need to use the package at runtime
# * `$pkg_prefix/ENVIRONMENT` - A list of environment keys and their values
# * `$pkg_prefix/ENVIRONMENT_SEP` - A list of Internal Field Separators for environment keys
# * `$pkg_prefix/EXPORTS` - A list of exported configuration keys and their public name
# * `$pkg_prefix/EXPOSES` - An array of `pkg_exports` for which ports that this package exposes
# * `$pkg_prefix/BINDS` - A list of services you connect to and keys that you expect to be exported
# * `$pkg_prefix/BINDS_OPTIONAL` - Same as `BINDS` but not required for the service to start
# * `$pkg_prefix/FILES` - blake2b checksums of all files in the package
# * `$pkg_prefix/LDFLAGS` - Any LDFLAGS for things that link against us
# * `$pkg_prefix/LD_RUN_PATH` - The LD_RUN_PATH for things that link against us
_build_metadata() {
  build_line "Building package metadata"
  local ld_run_path_part=()
  local ld_lib_part=()
  for lib in ${pkg_lib_dirs[@]}; do
    ld_run_path_part+=("${pkg_prefix}/$lib")
    ld_lib_part+=("-L${pkg_prefix}/$lib")
  done
  if [[ -n ${ld_run_path_part} ]]; then
    echo $(join_by ':' ${ld_run_path_part[@]}) > "$pkg_prefix/LD_RUN_PATH"
  fi
  if [[ -n ${ld_lib_part} ]]; then
    echo $(join_by ' ' ${ld_lib_part[@]}) > "$pkg_prefix/LDFLAGS"
  fi

  local cflags_part=()
  for inc in "${pkg_include_dirs[@]}"; do
    cflags_part+=("-I${pkg_prefix}/${inc}")
  done
  if [[ -n ${cflags_part} ]]; then
    echo $(join_by ' ' ${cflags_part[@]}) > "$pkg_prefix/CFLAGS"
  fi

  local cppflags_part=()
  for inc in "${pkg_include_dirs[@]}"; do
    cppflags_part+=("-I${pkg_prefix}/${inc}")
  done
  if [[ -n ${cppflags_part} ]]; then
    echo $(join_by ' ' ${cppflags_part[@]}) > "$pkg_prefix/CPPFLAGS"
  fi

  local cxxflags_part=()
  for inc in "${pkg_include_dirs[@]}"; do
    cxxflags_part+=("-I${pkg_prefix}/${inc}")
  done
  if [[ -n ${cxxflags_part} ]]; then
    echo $(join_by ' ' ${cxxflags_part[@]}) > "$pkg_prefix/CXXFLAGS"
  fi

  local pconfig_path_part=()
  if [[ ${#pkg_pconfig_dirs[@]} -eq 0 ]]; then
    # Plan doesn't define pkg-config paths so let's try to find them in the conventional locations
    local locations=(lib/pkgconfig share/pkgconfig)
    for dir in "${locations[@]}"; do
      if [[ -d "${pkg_prefix}/${dir}" ]]; then
        pconfig_path_part+=("${pkg_prefix}/${dir}")
      fi
    done
  else
    # Plan explicitly defines pkg-config paths so we don't provide defaults
    for inc in "${pkg_pconfig_dirs[@]}"; do
      pconfig_path_part+=("${pkg_prefix}/${inc}")
    done
  fi
  if [[ -n ${pconfig_path_part} ]]; then
    echo $(join_by ':' ${pconfig_path_part[@]}) > "$pkg_prefix/PKG_CONFIG_PATH"
  fi

  for env in ${!pkg_build_env[@]}; do
    echo "$env=${pkg_build_env[$env]}" >> "$pkg_prefix/BUILD_ENVIRONMENT"
  done

  for env in ${!pkg_env[@]}; do
    echo "$env=${pkg_env[$env]}" >> "$pkg_prefix/ENVIRONMENT"
  done

  for env_sep in ${!pkg_env_sep[@]}; do
    echo "$env_sep=${pkg_env_sep[$env_sep]}" >> "$pkg_prefix/ENVIRONMENT_SEP"
  done

  # Create PATH metadata for older versions of Habitat
  if [[ ${pkg_env[PATH]+abc} ]]; then
    echo "${pkg_env[PATH]}" > "$pkg_prefix/PATH"
  fi

  for export in "${!pkg_exports[@]}"; do
    echo "$export=${pkg_exports[$export]}" >> $pkg_prefix/EXPORTS
  done

  for bind in "${!pkg_binds[@]}"; do
    echo "$bind=${pkg_binds[$bind]}" >> $pkg_prefix/BINDS
  done

  for bind in "${!pkg_binds_optional[@]}"; do
    echo "$bind=${pkg_binds_optional[$bind]}" >> $pkg_prefix/BINDS_OPTIONAL
  done

  local port_part=""
  for export in "${pkg_exposes[@]}"; do
    if [[ ! ${pkg_exports[$export]+abc} ]]; then
      exit_with "Bad value in pkg_exposes; No pkg_export found matching key: ${export}"
    fi
    key=${pkg_exports[$export]}
    port=$($_rq_cmd -t < $PLAN_CONTEXT/default.toml "at \"${key}\"" | tr -d '"')
    if ! _port_is_valid $port; then
      exit_with "Bad pkg_export in pkg_exposes; Value of key \"${key}\" does not contain a valid TCP or UDP port number: ${port}"
    fi

    if [[ -z "$port_part" ]]; then
      port_part="$port";
    else
      port_part="$port_part $port";
    fi
  done
  if [[ -n "${port_part}" ]]; then
    echo $port_part > $pkg_prefix/EXPOSES
  fi

  if [[ ${#pkg_interpreters[@]} -gt 0 ]]; then
    local interpreters="$(printf "${pkg_prefix}/%s\n" ${pkg_interpreters[@]})"
    printf "%s\n" ${pkg_interpreters[@]} \
      | sed "s|^|${pkg_prefix}/|" > $pkg_prefix/INTERPRETERS
  fi

  local cutn="$(($(echo $HAB_PKG_PATH | grep -o '/' | wc -l)+2))"
  local deps

  deps="$(printf '%s\n' "${pkg_build_deps_resolved[@]}" \
    | cut -d "/" -f ${cutn}-)"
  if [[ -n "$deps" ]]; then
    echo "$deps" > $pkg_prefix/BUILD_DEPS
  fi
  deps="$(printf '%s\n' "${pkg_build_tdeps_resolved[@]}" \
    | cut -d "/" -f ${cutn}- | sort)"
  if [[ -n "$deps" ]]; then
    echo "$deps" > $pkg_prefix/BUILD_TDEPS
  fi
  deps="$(printf '%s\n' "${pkg_deps_resolved[@]}" | cut -d "/" -f ${cutn}-)"
  if [[ -n "$deps" ]]; then
    echo "$deps" > $pkg_prefix/DEPS
  fi
  deps="$(printf '%s\n' "${pkg_tdeps_resolved[@]}" \
    | cut -d "/" -f ${cutn}- | sort)"
  if [[ -n "$deps" ]]; then
    echo "$deps" > $pkg_prefix/TDEPS
  fi

  echo "$pkg_target" > $pkg_prefix/TARGET
  echo "${pkg_origin}/${pkg_name}/${pkg_version}/${pkg_release}" \
    >> $pkg_prefix/IDENT

  # Only generate `SVC_USER` & `SVC_GROUP` files if this package is a service.
  # We determine this by checking if there is a `hooks/run` script and/or
  # a set `$pkg_svc_run` value.
  if [[ -f "$PLAN_CONTEXT/hooks/run" || -n "${pkg_svc_run:-}" ]]; then
    echo "$pkg_svc_user" > $pkg_prefix/SVC_USER
    echo "$pkg_svc_group" > $pkg_prefix/SVC_GROUP
  fi

  # Generate the blake2b hashes of all the files in the package. This
  # is not in the resulting MANIFEST because MANIFEST is included!
  pushd "$CACHE_PATH" > /dev/null
  build_line "Generating blake2b hashes of all files in the package"
  find $pkg_prefix -type f \
    | $_sort_cmd \
    | hab pkg hash > ${pkg_name}_blake2bsums

  build_line "Generating signed metadata FILES"
  $HAB_BIN pkg sign --origin $pkg_origin ${pkg_name}_blake2bsums $pkg_prefix/FILES
  popd > /dev/null
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
  if [[ -d "$PLAN_CONTEXT/config" ]]; then
    if [[ -z "${HAB_CONFIG_EXCLUDE:-}" ]]; then
      # HAB_CONFIG_EXCLUDE not set, use defaults
      config_exclude_exts=("*.sw?" "*~" "*.bak")
    else
      IFS=',' read -a config_exclude_exts <<< "$HAB_CONFIG_EXCLUDE"
    fi
    find_exclusions=""
    for ext in "${config_exclude_exts[@]}"; do
      find_exclusions+=" ! -name $ext"
    done
    find "$PLAN_CONTEXT/config" $find_exclusions | while read FILE
    do
      if [[ -d "$FILE" ]]; then
        mkdir -p "$pkg_prefix${FILE#$PLAN_CONTEXT}"
      else
        cp "$FILE" "$pkg_prefix${FILE#$PLAN_CONTEXT}"
      fi
    done
    chmod 755 $pkg_prefix/config
  fi
  if [[ -d "$PLAN_CONTEXT/hooks" ]]; then
    cp -r "$PLAN_CONTEXT/hooks" $pkg_prefix
    chmod 755 $pkg_prefix/hooks
  fi
  if [[ -f "$PLAN_CONTEXT/default.toml" ]]; then
    cp "$PLAN_CONTEXT/default.toml" $pkg_prefix
  fi
  return 0
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
      # for debugging purposes, or under a `hab-director`.
      build_line "Writing ${pkg_prefix}/run script to run ${pkg_svc_run} as ${pkg_svc_user}:${pkg_svc_group}"
      cat <<EOT >> $pkg_prefix/run
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
do_default_strip() {
  build_line "Stripping unneeded symbols from binaries and libraries"
  find $pkg_prefix -type f -perm -u+w -print0 2> /dev/null \
    | while read -rd '' f; do
      case "$(file -bi "$f")" in
        *application/x-executable*) strip --strip-all "$f";;
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

  if [[ -z "$(echo ${pkg_build_deps[@]})" ]]; then
    local _build_deps_string="no build dependencies or undefined"
  else
    local _build_deps_string="\`$(printf "%s " ${pkg_build_deps[@]})\`"
  fi

  if [[ -z "$(echo ${pkg_deps[@]})" ]]; then
    local _deps_string="no runtime dependencies or undefined"
  else
    local _deps_string="\`$(printf "%s " ${pkg_deps[@]})\`"
  fi

  if [[ -z "$(echo ${pkg_interpreters[@]})" ]]; then
    local _interpreters_string="no interpreters or undefined"
  else
    local _interpreters_string="\`$(printf "%s " ${pkg_interpreters[@]})\`"
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

  cat <<-EOT >> $pkg_prefix/MANIFEST
# $pkg_origin / $pkg_name
$pkg_description

* __Maintainer__: $pkg_maintainer
* __Version__: $pkg_version
* __Release__: $pkg_release
* __Architecture__: $pkg_arch
* __System__: $pkg_sys
* __Target__: $pkg_target
* __Upstream URL__: $_upstream_url_string
* __License__: $(printf "%s " ${pkg_license[@]})
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
$(cat $PLAN_CONTEXT/plan.sh)
\`\`\`
EOT
  return 0
}

# **Internal** Create the package artifact with `tar`/`hab pkg sign`
_generate_artifact() {
  build_line "Generating package artifact"
  local tarf="$(dirname $pkg_artifact)/.$(basename ${pkg_artifact/%.${_artifact_ext}/.tar})"
  local xzf="${tarf}.xz"

  mkdir -pv "$(dirname "$pkg_artifact")"
  rm -fv $tarf $xzf $pkg_artifact
  $_tar_cmd -cf $tarf $pkg_prefix
  $_xz_cmd --compress -6 --threads=0 $tarf
  $HAB_BIN pkg sign --origin $pkg_origin $xzf $pkg_artifact
  rm -f $tarf $xzf
}

_prepare_build_outputs() {
  local plan_owner
  _pkg_sha256sum=$($_shasum_cmd $pkg_artifact | cut -d " " -f 1)
  _pkg_blake2bsum=$($HAB_BIN pkg hash $pkg_artifact | cut -d " " -f 1)
  plan_owner="$(stat -c '%u:%g' "$PLAN_CONTEXT/plan.sh")"

  mkdir -pv "$pkg_output_path"
  # Attempt to set user/group ownership to the same as the ownership of the
  # `plan.sh` file. If the `chown` fails, then don't stop the build--this is
  # only best effort.
  chown "$plan_owner" "$pkg_output_path" || true
  cp -v "$pkg_artifact" "$pkg_output_path"/
  chown "$plan_owner" "$pkg_output_path/$(basename "$pkg_artifact")" || true

  cat <<-EOF > "$pkg_output_path"/last_build.env
pkg_origin=$pkg_origin
pkg_name=$pkg_name
pkg_version=$pkg_version
pkg_release=$pkg_release
pkg_ident=${pkg_origin}/${pkg_name}/${pkg_version}/${pkg_release}
pkg_artifact=$(basename $pkg_artifact)
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

# # Main Flow

# Parse depot flag (-u)
OPTIND=2
while getopts "u:" opt; do
  case "${opt}" in
    u)
      HAB_DEPOT_URL=$OPTARG
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

# Expand the context path to an absolute path
PLAN_CONTEXT="$(abspath "$PLAN_CONTEXT")"
# Set the initial source root to be the same as the Plan context directory.
# This assumes that your application source is local and your Plan exists with
# your code.
SRC_PATH="$PLAN_CONTEXT"
# Expand the path of this program to an absolute path
THIS_PROGRAM=$(abspath "$0")

# Now to ensure a `plan.sh` exists where we expect it. There are 2 possible
# candidate locations relative to the `$PLAN_CONTEXT` directory: a `./plan.sh`
# or a `./habitat/plan.sh`. Only one or the other location is allowed so that
# a Plan author isn't confused if they edit one to have this program read
# the other.

# We'll make sure that both are not present, and if so abort.
if [[ -f "$PLAN_CONTEXT/plan.sh" && -f "$PLAN_CONTEXT/habitat/plan.sh" ]];then
  places="$PLAN_CONTEXT/plan.sh and $PLAN_CONTEXT/habitat/plan.sh"
  exit_with "A Plan file was found at $places. Only one is allowed at a time" 42
fi
# We check if the provided path has a `plan.sh` in it in either location. If
# not, we'll quickly bail.
if [[ ! -f "$PLAN_CONTEXT/plan.sh" ]]; then
  if [[ -f "$PLAN_CONTEXT/habitat/plan.sh" ]]; then
    # As the `plan.sh` is in a deeper subdirectory, we'll update the
    # `$PLAN_CONTEXT` directory to be relative to the actual `plan.sh` file.
    PLAN_CONTEXT="$PLAN_CONTEXT/habitat"
  else
    places="$PLAN_CONTEXT/plan.sh or $PLAN_CONTEXT/habitat/plan.sh"
    exit_with "Plan file not found at $places" 42
  fi
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
  pkg_svc_run="$(echo $pkg_svc_run | sed "s|@__pkg_name__@|$pkg_name|g")"
fi

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
pkg_svc_static_path="$pkg_svc_path/static"

# Set the package artifact name
_artifact_ext="hart"
pkg_artifact="$HAB_CACHE_ARTIFACT_PATH/${pkg_origin}-${pkg_name}-${pkg_version}-${pkg_release}-${pkg_target}.${_artifact_ext}"

# Run `do_begin`
build_line "$_program setup"
do_begin

# Determine if we have all the commands we need to work
_find_system_commands

# Ensure that the origin key is available for package signing
_ensure_origin_key_present

_determine_hab_bin

_resolve_dependencies

# Set up runtime environment
_set_environment

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

# Write the manifest
_build_manifest

# Render the linking and dependency files
_build_metadata

# Generate the artifact and write to artifact cache
_generate_artifact

# Copy produced artifact to a local relative directory
_prepare_build_outputs

# Cleanup
build_line "$_program cleanup"
do_end

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
_on_exit  0
