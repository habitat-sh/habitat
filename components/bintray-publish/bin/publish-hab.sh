#!/bin/bash
#
# # Usage
#
# ```sh
# $ publish-hab [FLAGS] [OPTIONS] <PKG_IDENT_OR_ARTIFACT>
# ```
#
# See the `print_help()` function below for complete usage instructions.
#
# # License and Copyright
#
# ```
# Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
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

# Fail if there are any unset variables and whenever a command returns a
# non-zero exit code.
set -eu

# If the variable `$DEBUG` is set, then print the shell commands as we execute.
if [ -n "${DEBUG:-}" ]; then
  set -x
  export DEBUG
fi


# ## Help/Usage functions

# **Internal** Prints help and usage information. Straight forward, no?
print_help() {
  echo -- "$program $version

$author

Habitat CLI Bintray Publisher

USAGE:
        $program [FLAGS] [OPTIONS] <HART_FILE>

COMMON FLAGS:
    -h  Prints this message
    -o  Specify the Bintray organization to publish to (default: habitat)
    -r  Specify the Bintray repo to publish to (default: stable)
    -s  Skip publishing the artifact (upload only)
    -V  Prints version information

ARGS:
    <HART_FILE> A path to a local Habitat artifact
"
}


# **Internal** Check that the command exists, 0 if it does, 1 if it does not.
#
# ```sh
# exists gsha256sum
# ```
#
# Would return 0 if gsha256sum exists, 1 if it does not.
exists() {
  if command -v "$1" >/dev/null 2>&1
  then
    return 0
  else
    return 1
  fi
}

# **Internal** Exit the program with an error message and a status code.
#
# ```
# exit_with "Something bad went down" 55
# ```
exit_with() {
  case "${TERM:-}" in
    *term | xterm-* | rxvt | screen | screen-*)
      echo -e "\033[1;31mERROR: \033[1;37m$1\033[0m"
      ;;
    *)
      echo "ERROR: $1"
      ;;
  esac
  exit "$2"
}

# **Internal** Ensures that the correct versions of key system commands are
# able to be used by this program. If we cannot find suitable versions, we will
# abort early.
#
# The following variables are set which contain an absolute path to the desired
# command:
#
# * `$_gzip_cmd` (gzip on system)
# * `$_hab_cmd` (hab CLI for signing artifacts)
# * `$_tar_cmd` (GNU version of tar)
# * `$_zip_cmd` (zip on system)
#
# Note that all of the commands noted above are considered internal
# implementation details and are subject to change with little to no notice,
# which means the variables such as `$_tar_cmd` should **not** be used directly
# by Plan authors. The leading underscore denotes an internal/private variable
# or function.

# If the commands are not found, `exit_with` is called and the program is
# terminated.
_find_system_commands() {
  if exists gzip; then
    _gzip_cmd=$(command -v gzip)
  else
    exit_with "We require gzip to compress Linux binaries; aborting" 1
  fi

  if exists hab; then
    _hab_cmd=$(command -v hab)
  else
    exit_with "We require hab to sign artifacts; aborting" 1
  fi

  if exists jfrog; then
    _jfrog_cmd="env JFROG_CLI_OFFER_CONFIG=false $(command -v jfrog)"
  else
    exit_with "We require jfrog to publish artifacts to Bintray; aborting" 1
  fi

  if tar --version 2>&1 | grep -q 'GNU tar'; then
    _tar_cmd=$(command -v tar)
  else
    exit_with "We require GNU tar for long path support; aborting" 1
  fi

  if exists zip; then
    _zip_cmd=$(command -v zip)
  else
    exit_with "We require zip to compress Mac binaries; aborting" 1
  fi
}

# **Internal** Print a line of output. Takes the rest of the line as its only
# argument.
#
# ```sh
# info "Running command"
# ```
info() {
  case "${TERM:-}" in
    *term | xterm-* | rxvt | screen | screen-*)
      printf -- "   \033[1;36m%s: \033[1;37m%s\033[0m\n" "${program:-unknown}" "${1:-}"
      ;;
    *)
      printf -- "   ${program:-unknown}: %s\n" "${1:-}"
      ;;
  esac
  return 0
}

_build_slim_release() {
  info "Extracting Habitat package $target_hart"
  if [ ! -e "$target_hart" ]; then
    exit_with ".hart file not found at given path: $target_hart" 1
  fi

  # We will extract the package from the .hart archive into this directory.
  extract_dir="${tmp_root}/extract"
  mkdir -p "${extract_dir}"

  # Strip the header from the hart file (first 6 lines), then extract
  # the package contents to $extract_dir.
  #
  # (--strip-components=6 will strip off the
  # hab/pkgs/core/hab/$VERSION/$RELEASE portion of the path, because
  # that's completely unnecessary for what we're doing here.)
  tail -n+6 "${target_hart}" | \
      tar --directory "${extract_dir}" \
          --extract \
          --xz \
          --strip-components=6

  if [[ $(find "${extract_dir}" \( -name hab -or -name hab.exe \) -type f | wc -l) -ne 1 ]]; then
    exit_with "$target_hart did not contain a \`hab' binary" 2
  fi

  local hab_binary
  hab_binary="$(find "$extract_dir" \( -name hab -or -name hab.exe \) -type f)"
  pkg_target="$(tr --delete '\r' < "${extract_dir}"/TARGET)"
  pkg_arch="$(echo "$pkg_target" | cut -d '-' -f 1)"
  pkg_kernel="$(echo "$pkg_target" | cut -d '-' -f 2)"
  pkg_ident="$(tr --delete '\r' < "$extract_dir"/IDENT)"
  pkg_origin="$(echo "$pkg_ident" | cut -d '/' -f 1)"
  pkg_name="$(echo "$pkg_ident" | cut -d '/' -f 2)"
  pkg_version="$(echo "$pkg_ident" | cut -d '/' -f 3)"
  pkg_release="$(echo "$pkg_ident" | cut -d '/' -f 4)"
  local archive_name build_dir pkg_dir
  archive_name="hab-$(echo "$pkg_ident" | cut -d '/' -f 3-4 | tr '/' '-')-$pkg_target"
  build_dir="$tmp_root/build"
  pkg_dir="$build_dir/${archive_name}"

  info "Copying $hab_binary to $(basename "$pkg_dir")"
  mkdir -p "$pkg_dir"
  mkdir -p "$start_dir/results"

  if [[ $pkg_target == *"windows" ]]; then
    for file in $(dirname "$hab_binary")/*; do cp -p "$file" "$pkg_dir/";done
  else
    cp -p "$hab_binary" "$pkg_dir/$(basename "$hab_binary")"
  fi

  info "Compressing \`hab' binary"
  pushd "$build_dir" >/dev/null
  case "$pkg_target" in
    *-linux | *-linux-kernel2)
      pkg_artifact="$start_dir/results/${archive_name}.tar.gz"
      local tarball
      tarball="$build_dir/$(basename "${pkg_artifact%.gz}")"
      $_tar_cmd cf "$tarball" "$(basename "$pkg_dir")"
      rm -fv "$pkg_artifact"
      $_gzip_cmd -9 -c "$tarball" > "$pkg_artifact"
      ;;
    *-darwin | *-windows)
      pkg_artifact="$start_dir/results/${archive_name}.zip"
      rm -fv "$pkg_artifact"
      $_zip_cmd -9 -r "$pkg_artifact" "$(basename "$pkg_dir")"
      ;;
    *)
      exit_with "$target_hart has unknown TARGET=$pkg_target" 3
      ;;
  esac
  popd >/dev/null
  pushd "$(dirname "$pkg_artifact")" >/dev/null
  sha256sum "$(basename "$pkg_artifact")" > "${pkg_artifact}.sha256sum"
  popd
}

_publish_slim_release() {
  bintray_pkg="hab-${pkg_target}"
  bintray_version="$(echo "$pkg_ident" | cut -d '/' -f 3-4 | tr '/' '-')"
  bintray_endpoint="$BINTRAY_ORG/$BINTRAY_REPO/$bintray_pkg/$bintray_version"
  bintray_path="$pkg_kernel/$pkg_arch"

  info "Checking to see if Bintray package $bintray_pkg already exists"

  # Note: as coded, this first call can fail if the credentials are bad.
  if $_jfrog_cmd bt package-show \
    --user="$BINTRAY_USER" \
    --key="$BINTRAY_KEY" \
    "$BINTRAY_ORG/$BINTRAY_REPO/$bintray_pkg" ; then
    info "$bintray_pkg already exists. No need to create it."
  else
    info "$bintray_pkg does not exist. Creating it now."
    $_jfrog_cmd bt package-create \
      --user="$BINTRAY_USER" \
      --key="$BINTRAY_KEY" \
      --licenses=Apache-2.0 \
      --vcs-url=https://github.com/habitat-sh/habitat \
      --issuetracker-url=https://github.com/habitat-sh/habitat/issues \
      --pub-dn=false \
      --pub-stats=false \
      --website-url=https://www.habitat.sh \
      "$BINTRAY_ORG/$BINTRAY_REPO/$bintray_pkg"
  fi

  for a in $pkg_artifact ${pkg_artifact}.sha256sum; do
    info "Uploading $(basename "$a") to $bintray_endpoint"
    $_jfrog_cmd bt upload \
      --user="$BINTRAY_USER" \
      --key="$BINTRAY_KEY" \
      "$a" \
      "$bintray_endpoint" \
      "$bintray_path"/
  done

  info "Signing files in $bintray_endpoint"
  $_jfrog_cmd bt gpg-sign-ver \
    --user="$BINTRAY_USER" \
    --key="$BINTRAY_KEY" \
    --passphrase="$BINTRAY_PASSPHRASE" \
    "$bintray_endpoint"

  if [ -n "${SKIP_PUBLISH:-}" ]; then
    return 0
  fi

  info "Publishing version $bintray_endpoint"
  $_jfrog_cmd bt version-publish \
    --user="$BINTRAY_USER" \
    --key="$BINTRAY_KEY" \
    "$bintray_endpoint"
}

# **Internal** Main program.
_main() {

  _build_slim_release
  _publish_slim_release

  cat <<-EOF > "$start_dir"/results/last_build.env
pkg_origin=$pkg_origin
pkg_name=$pkg_name
pkg_version=$pkg_version
pkg_release=$pkg_release
pkg_target=$pkg_target
pkg_ident=${pkg_origin}/${pkg_name}/${pkg_version}/${pkg_release}
pkg_artifact=$(basename "$pkg_artifact")
EOF

  info
  info "Artifact: $pkg_artifact"
  info "Build Report: $start_dir/results/last_build.env"
  info
}


# # Main Flow

# ## Default variables

BINTRAY_ORG=habitat
BINTRAY_REPO=stable

# The current version of this program
version='@version@'
# The author of this program
author='@author@'
# The short version of the program name which is used in logging output
program=$(basename "$0")
# The initial working directory when the program started
start_dir="$(pwd)"

# ## CLI Argument Parsing

# Parse command line flags and options.
while getopts "o:r:Vhs" opt; do
  case $opt in
    o)
      BINTRAY_ORG=$OPTARG
      ;;
    r)
      BINTRAY_REPO=$OPTARG
      ;;
    s)
      SKIP_PUBLISH=true
      ;;
    V)
      echo "$program $version"
      exit 0
      ;;
    h)
      print_help
      exit 0
      ;;
    \?)
      print_help
      exit_with "Invalid option: -$OPTARG" 1
      ;;
  esac
done
# Shift off all parsed token in `$*` so that the subcommand is now `$1`.
shift "$((OPTIND - 1))"

if [[ -z "${1:-}" ]]; then
  print_help
  exit_with "Required argument: <PKG_IDENT_OR_ARTIFACT>" 2
fi
if [[ -z "${BINTRAY_USER:-}" ]]; then
  print_help
  exit_with "Required environment variable: BINTRAY_USER" 2
fi
if [[ -z "${BINTRAY_KEY:-}" ]]; then
  print_help
  exit_with "Required environment variable: BINTRAY_KEY" 2
fi
if [[ -z "${BINTRAY_PASSPHRASE:-}" ]]; then
  print_help
  exit_with "Required environment variable: BINTRAY_PASSPHRASE" 2
fi
: ${SKIP_PUBLISH:=}

target_hart="$1"

tmp_root="$(mktemp -d -t "${program}-XXXX")"
cleanup() {
    original_exit=$?
    rm -rf "${tmp_root}"
    exit $original_exit
}

trap cleanup INT TERM EXIT

_find_system_commands
_main
