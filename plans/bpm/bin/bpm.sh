#!/bin/sh
#
# # Usage
#
# ```sh
# $ bpm install chef/bldr-studio
# $ bpm exec chef/bash bash --version
# ```
#
# # Synopsis
#
# blah
#
# # License and Copyright
#
# ```
# Copyright: Copyright (c) 2016 Chef Software, Inc.
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
fi


# ## Help/Usage functions

# **Internal** Prints help and usage information. Straight forward, no?
print_help() {
  printf -- "$program $version

$author

Bldr Package Manager

USAGE:
        $program [COMMON_FLAGS] <SUBCOMMAND> [ARG ..]

COMMON FLAGS:
    -h  Prints this message
    -q  Prints less output for better use in scripts
    -v  Prints more verbose output
    -V  Prints version information

SUBCOMMANDS:
    binlink   Creates a symlink for a package binary in a common 'PATH' location
    exec      Executes a command using the 'PATH' context of an installed package
    help      Prints this message
    install   Installs a package
    pkgpath   Prints the path to a package
    version   Prints version information

ENVIRONMENT VARIABLES:
    QUIET         Prints less output (\`-q' flag takes precedence)
    VERBOSE       Prints more verbose output (\`-v' flag takes precedence)

SUBCOMMAND HELP:
        $program <SUBCOMMAND> -h
"
}

print_binlink_help() {
  printf -- "${program}-binlink $version

$author

Bldr Package Manager - create a symlink for a package binary into a common
'PATH' location

USAGE:
        $program [COMMON_FLAGS] binlink [FLAGS] [OPTIONS] <PKG_IDENT> <BINARY>

FLAGS:
    -h  Prints this message

OPTIONS:
    -d <DEST_DIR>   Sets the destination directory (default: \`/usr/bin')

EXAMPLES:

    # Symlink 'bin/bash' to '/usr/bin/bash' from a specific package release
    $program binlink acme/bash/4.3.42/20160126184157 bash

    # Symlink 'bin/busybox' to a custom path '/bin/busybox'
    $program binlink -d /bin chef/busybox busybox

GENERAL HELP:
        $program help

"
}

print_exec_help() {
  printf -- "${program}-exec $version

$author

Bldr Package Manager - execute a command using the 'PATH'
context of an installed package

USAGE:
        $program [COMMON_FLAGS] exec <PKG_IDENT> <COMMAND> [ARG ..]

EXAMPLES:

    # Execute a command against a specific release
    $program exec acme/bash/4.3.42/20160126184157 bash --version

    # Execute a command against the latest installed release of a package
    $program exec chef/bootstrap-toolchain wget --version

GENERAL HELP:
        $program help

"
}

print_install_help() {
  printf -- "${program}-install $version

$author

Bldr Package Manager - installing packages

USAGE:
        $program [COMMON_FLAGS] install [FLAGS] [OPTIONS] <PKG_IDENT>

FLAGS:
    -h  Prints this message

OPTIONS:
    -u <BLDR_REPO>  Sets a Bldr repository URL

ENVIRONMENT VARIABLES:
    BLDR_REPO     Sets a Bldr repository (\`-u' option takes precedence)

EXAMPLES:

    # Install a specific release
    $program install chef/zlib/1.2.8/20160104212444

    # Install the latest release of a package
    $program install chef/bootstrap-toolchain

    # Install the latest release of a package from a custom repository
    $program install -u http://127.0.0.1:9633 chef/bootstrap-toolchain

GENERAL HELP:
        $program help

"
}

print_pkgpath_help() {
  printf -- "${program}-pkgpath $version

$author

Bldr Package Manager - print the path to an installed package

USAGE:
        $program [COMMON_FLAGS] pkgpath <PKG_IDENT>

EXAMPLES:

    # Print the path to a specific installed release of a package
    $program pkgpath acme/bash/4.3.42/20160126184157

    # Print the path to the latest installed release of a package
    $program pkgpath chef/bootstrap-toolchain

GENERAL HELP:
        $program help

"
}


# ## Subcommand functions
#
# These are the implmentations for each subcommand in the program.

subcommand_binlink() {
  local opt
  local pkg_ident
  local dest_dir

  OPTIND=1
  # Parse command line flags and options.
  while getopts ":d:h" opt; do
    case $opt in
      d)
        dest_dir=$OPTARG
        ;;
      h)
        print_binlink_help
        exit 0
        ;;
      \?)
        print_binlink_help
        exit_with "Invalid option: -$OPTARG" 1
        ;;
    esac
  done
  # Shift off all parsed token in `$*` so that the subcommand is now `$1`.
  shift "$((OPTIND - 1))"

  : ${dest_dir:=/usr/bin}

  if [ -z "${1:-}" ]; then
    warn "Missing package as an argument to '$program binlink'"
    print_binlink_help
    exit_with "Missing package argument" 5
  fi
  local pkg_ident_arg=${1:-}
  shift
  if [ -z "${1:-}" ]; then
    warn "Missing binary as an argument to '$program binlink $pkg_ident_arg'"
    print_binlink_help
    exit_with "Missing command argument" 6
  fi
  local binary=${1:-}
  shift
  if ! pkg_ident="$(latest_installed_package $pkg_ident_arg)"; then
    exit_with "Installed package could not be found for: $pkg_ident_arg" 7
  fi
  local pkg_path=$(subcommand_pkgpath $pkg_ident)
  if [ ! -f "$pkg_path/PATH" ]; then
    exit_with "Package $pkg_ident does not expose a 'PATH' entry" 8
  fi

  local abs_binary
  for p in $($bb cat $pkg_path/PATH | $bb tr ':' ' '); do
    if [ -x "$p/$binary" ]; then
      abs_binary="$p/$binary"
      break
    fi
  done
  if [ -z "${abs_binary:-}" ]; then
    exit_with "Binary '$binary' not found in 'PATH' entry for $pkg_ident" 9
  fi

  $bb ln -snf $abs_binary $dest_dir/$binary

  info "Binary '$binary' from $pkg_ident symlinked to $dest_dir/$binary"
}

subcommand_exec() {
  local opt
  local pkg_ident
  local dest_dir

  OPTIND=1
  # Parse command line flags and options.
  while getopts ":h" opt; do
    case $opt in
      h)
        print_exec_help
        exit 0
        ;;
      \?)
        print_exec_help
        exit_with "Invalid option: -$OPTARG" 1
        ;;
    esac
  done
  # Shift off all parsed token in `$*` so that the subcommand is now `$1`.
  shift "$((OPTIND - 1))"

  if [ -z "${1:-}" ]; then
    warn "Missing package as an argument to '$program exec'"
    print_exec_help
    exit_with "Missing package argument" 5
  fi
  local pkg_ident_arg=${1:-}
  shift
  if [ -z "${1:-}" ]; then
    warn "Missing command as an argument to '$program exec $pkg_ident_arg'"
    print_exec_help
    exit_with "Missing command argument" 6
  fi
  local cmd=${1:-}
  shift
  if ! pkg_ident="$(latest_installed_package $pkg_ident_arg)"; then
    exit_with "Installed package could not be found for: $pkg_ident_arg" 6
  fi

  set_path $pkg_ident

  if [ -z "$QUIET" ]; then
    info "Using: $pkg_ident"
    info "Setting: PATH=$PATH"
    info "Running: '$cmd $*'"
  fi

  # Finally, become the `$cmd` process and pass along remaining arguments
  exec $cmd $*
}

subcommand_install() {
  local opt
  local pkg_indent
  local pkg_local
  local latest

  OPTIND=1
  # Parse command line flags and options.
  while getopts ":u:h" opt; do
    case $opt in
      u)
        BLDR_REPO=$OPTARG
        ;;
      h)
        print_install_help
        exit 0
        ;;
      \?)
        print_install_help
        exit_with "Invalid option: -$OPTARG" 1
        ;;
    esac
  done
  # Shift off all parsed token in `$*` so that the subcommand is now `$1`.
  shift "$((OPTIND - 1))"

  if [ -z "${1:-}" ]; then
    warn "Missing package as an argument to '$program install'"
    print_install_help
    exit_with "Missing package argument" 5
  fi
  # If a remote package version could not be determined, ensure that
  # `$pkg_ident` is empty--this signals a remote failure
  if ! pkg_ident="$(latest_remote_package $1)"; then
    pkg_ident=""
  fi
  # If a local package is installed and it is newer than the latest version
  # that is available remotely, use the local version.
  if pkg_local="$(latest_installed_package $1 quietly)"; then
    latest="$(printf -- "$pkg_ident\n$pkg_local\n" \
      | $cu --coreutils-prog=sort --version-sort -r | $bb head -n 1)"
    if [ "$latest" = "$pkg_local" ]; then
      if [ -n "$VERBOSE" ]; then
        info "Local package $pkg_local is newer than remote $pkg_ident, skipping"
      fi
      return 0
    fi
  fi
  # If a suitable local package was not found above (and triggered an early
  # return), and `$pkg_ident` is empty, then we have not found a remote or
  # local pacakge. Time to die.
  if [ -z "$pkg_ident" ]; then
    exit_with "Remote package could not be found for: $1" 6
  fi

  # Install the package and each of its direct and transitive runtime
  # dependencies
  install_package $pkg_ident
  install_package_tdeps $pkg_ident
}

subcommand_pkgpath() {
  local opt
  local pkg_ident

  OPTIND=1
  # Parse command line flags and options.
  while getopts ":h" opt; do
    case $opt in
      h)
        print_pkgpath_help
        exit 0
        ;;
      \?)
        print_pkgpath_help
        exit_with "Invalid option: -$OPTARG" 1
        ;;
    esac
  done
  # Shift off all parsed token in `$*` so that the subcommand is now `$1`.
  shift "$((OPTIND - 1))"

  if [ -z "${1:-}" ]; then
    warn "Missing package as an argument to '$program pkgpath'"
    print_pkgpath_help
    exit_with "Missing package argument" 5
  fi
  local pkg_ident_arg=${1:-}
  shift
  if ! pkg_ident="$(latest_installed_package $pkg_ident_arg)"; then
    exit_with "Installed package could not be found for: $pkg_ident_arg" 6
  fi

  echo "$BLDR_PKG_ROOT/$pkg_ident"
}


# ## Private/Internal helper functions
#
# These functions are part of the private/internal API of this program and
# should **not** be used externally by other programs. Their behaviors and
# names can change with little to no warning and no direct support can be
# provided as a result. Thank you for your understanding--maintaining a tiny
# but robust public interface is not an easy task.

# **Internal** Print a line of output. Takes the rest of the line as its only
# argument.
#
# ```sh
# info "Running command"
# ```
info() {
  if [ -n "${QUIET:-}" ]; then
    return 0
  fi

  case "${TERM:-}" in
    *term | xterm-* | rxvt | screen | screen-*)
      printf -- "   \033[1;36m${program:-unknown}: \033[1;37m$1\033[0m\n"
      ;;
    *)
      printf -- "   ${program:-unknown}: $1\n"
      ;;
  esac
  return 0
}

# **Internal** Print a warning line on stderr. Takes the rest of the line as
# its only argument.
#
# ```sh
# warn "Things are about to go bad"
# ```
warn() {
  if [ -n "${QUIET:-}" ]; then
    return 0
  fi

  case "${TERM:-}" in
    *term | xterm-* | rxvt | screen | screen-*)
      >&2 printf -- "   \033[1;36m${program:-unknown}: \033[1;33mWARN \033[1;37m$1\033[0m\n"
      ;;
    *)
      >&2 printf -- "   ${program:-unknown}: WARN $1\n"
      ;;
  esac
  return 0
}

# **Internal** Exit the program with an error message and a status code.
#
# ```sh
# exit_with "Something bad went down" 55
# ```
exit_with() {
  case "${TERM:-}" in
    *term | xterm-* | rxvt | screen | screen-*)
      printf -- "\033[1;31mERROR: \033[1;37m$1\033[0m\n"
      ;;
    *)
      printf -- "ERROR: $1\n"
      ;;
  esac
  exit $2
}

# **Internal** Trim leading and trailing whitespace.
#
# Thanks to: http://stackoverflow.com/questions/369758/how-to-trim-whitespace-from-bash-variable
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

# **Internal** Return the latest release of a package in the remote repository
# on stdout. If a fully qualified package identifier is given, simply return
# it.
#
# ```sh
# latest_remote_package chef/nginx
# # chef/nginx/1.8.0/20150911120000
# latest_remote_package chef/zlib/1.2.8/20160104212444
# # chef/zlib/1.2.8/20160104212444
# ```
#
# Will return 0 if a fully qualified package identifier could be determined,
# and 1 otherwise. A message will be printed to stderr explaining that no
# package release could be found.
latest_remote_package() {
  local latest_package_flags="$(echo $1 | $bb grep -o '/' | $bb wc -l)"
  case $(trim $latest_package_flags) in
    "3")
      # Nothing to do, the `$pkg_ident` is fully qualified
      echo $1
      ;;
    "2"|"1")
      local result="$(\
        $bb env -u http_proxy $bb wget "$BLDR_REPO/pkgs/$1" -O- -q | \
        $jq -r 'last | .origin + "/" + .name + "/" + .version + "/" + .release')"
      if [ -n "$result" ]; then
        echo $result
      else
        warn "Could not find a suitable remote package release for $1"
        return 1
      fi
      ;;
    *)
      warn "Could not find a suitable remote package release for $1"
      return 1
      ;;
  esac

  return 0
}

# **Internal** Return the latest release of a package locally installed on
# stdout.
#
# ```sh
# latest_installed_package acme/nginx
# # acme/nginx/1.8.0/20150911120000
# latest_installed_package acme/nginx/1.8.0
# # acme/nginx/1.8.0/20150911120000
# latest_installed_package acme/nginx/1.8.0/20150911120000
# # acme/nginx/1.8.0/20150911120000
# ```
#
# Will return 0 if a fully qualified package identifier could be determined and
# 1 if a package cannot be found. A message will be printed to stderr
# explaining that no package was found.
latest_installed_package() {
  local quietly="${2:-}"
  if [ ! -d "$BLDR_PKG_ROOT/$1" ]; then
    if [ -z "$quietly" ]; then
      warn "No installed packages of '$1' were found"
    fi
    return 1
  fi

  # Count the number of slashes, and use it to make a choice
  # about what to return as the latest package.
  local latest_package_flags="$(echo $1 | $bb grep -o '/' | $bb wc -l)"
  local result
  case $(trim $latest_package_flags) in
    "3")
      result="$BLDR_PKG_ROOT/$1"
      ;;
    "2")
      result="$($bb find $BLDR_PKG_ROOT/$1 -maxdepth 1 -type d \
        | $cu --coreutils-prog=sort --version-sort -r | $bb head -n 1)"
      ;;
    "1")
      result="$($bb find $BLDR_PKG_ROOT/$1 -maxdepth 2 -type d \
        | $cu --coreutils-prog=sort --version-sort -r | $bb head -n 1)"
      ;;
  esac
  if [ -z "$result" -a ! -f "$result/MANIFEST" ]; then
    if [ -z "$quietly" ]; then
      warn "Could not find a suitable installed package for '$1'"
    fi
    return 1
  else
    echo "$result" | $bb sed "s,^$BLDR_PKG_ROOT/,,"
    return 0
  fi
}

# **Internal** Installs a package from a package repository, represented by the
# given package identifier.
#
# Note that a fully qualified package identifier must be provided, that is
# `<ORIGIN>/<NAME>/<VERSION>/<RELEASE>`.
#
# ```sh
# install_package chef/zlib/1.2.8/20160104212444
# ```
install_package() {
  local pkg_ident=$1
  local pkg_source="$BLDR_REPO/pkgs/$pkg_ident/download"
  local pkg_filename="$BLDR_PKG_CACHE/$(echo $pkg_ident | $bb tr '/' '-').bldr"

  if [ -n "$QUIET" ]; then
    local v=
    local wui="-q"
  elif [ -n "$VERBOSE" ]; then
    local v="-v"
    local wui=
  else
    local v=
    local wui=
  fi

  if latest_installed_package $pkg_ident quietly > /dev/null; then
    if [ -n "$VERBOSE" ]; then
      warn "Skipping install as $pkg_ident is installed"
    fi
  else
    info "Installing $pkg_ident"

    $bb mkdir -p $v $BLDR_PKG_CACHE

    # Add a trap to clean up any interrupted file downloads and failed
    # extractions. These signal traps will be cleared once extraction is
    # completed.
    trap '$bb rm -f $pkg_filename; exit $?' INT TERM EXIT

    info "Downloading $($bb basename $pkg_filename)"
    $bb wget $pkg_source -O $pkg_filename $wui

    info "Unpacking $($bb basename $pkg_filename)"
    local gpg_cmd="$gpg --homedir $BLDR_GPG_CACHE --decrypt $pkg_filename"
    if [ -n "$VERBOSE" ]; then $gpg_cmd; else $gpg_cmd 2>/dev/null; fi \
      | $bb tar x -C $FS_ROOT/

    # Clear the file download and extraction clean trap
    trap - INT TERM EXIT
  fi
}

# **Internal** Installs all direct and transitive dependencies for a package
# from a package repository, represented by the given package identifier.
#
# Note that a fully qualified package identifier must be provided, that is
# `<ORIGIN>/<NAME>/<VERSION>/<RELEASE>`.
#
# ```sh
# install_package_tdeps chef/zlib/1.2.8/20160104212444
# ```
install_package_tdeps() {
  local pkg_ident=$1

  # Install each entry in the package's `TDEPS` file which constitute the
  # entire set of runtime dependencies--direct and transitive.
  if [ -f "$BLDR_PKG_ROOT/$pkg_ident/TDEPS" ]; then
    for dep_ident in $($bb cat $BLDR_PKG_ROOT/$pkg_ident/TDEPS); do
      install_package $dep_ident
    done
  fi
}

# **Internal** Sets the `PATH` environment variable. The `PATH` will be
# constructed by inspecting the package metadata of the target package and each
# of its dependencies' metadata. Any `PATH` metadata enries will be added from
# the direct depencies first (in declared order) and then from any remaining
# transitive depdendencies last (in lexically sorted order).
set_path() {
  local path_parts
  local dep_ident
  local dep_path
  local pkg_path="$BLDR_PKG_ROOT/$1"

  # Start with the `PATH` entry from this package, if it exists
  if [ -f "$pkg_path/PATH" ]; then
    path_parts="$($bb cat $pkg_path/PATH)"
  else
    path_parts=""
  fi

  # Only add dependecies' `PATH` entries if this package has depdendencies
  if [ -f "$pkg_path/DEPS" ]; then
    # Loop through each `DEPS` entry and add the `PATH` entry for each direct
    # dependency (if it exists)
    for dep_ident in $($bb cat $pkg_path/DEPS); do
      if [ -f "$BLDR_PKG_ROOT/$dep_ident/PATH" ]; then
        dep_path="$($bb cat $BLDR_PKG_ROOT/$dep_ident/PATH)"
        if [ -z "$path_parts" ]; then
          path_parts="$dep_path"
        else
          path_parts="$(return_or_append_to_path_set "$dep_path" "$path_parts")"
        fi
      fi
    done
    # Loop through each `TDEPS` entry and add the `PATH` entry for each
    # dependency (if it exists). If the entry already exists, skip it
    for dep_ident in $($bb cat $pkg_path/TDEPS); do
      if [ -f "$BLDR_PKG_ROOT/$dep_ident/PATH" ]; then
        dep_path="$($bb cat $BLDR_PKG_ROOT/$dep_ident/PATH)"
        if [ -z "$path_parts" ]; then
          path_parts="$dep_path"
        else
          path_parts="$(return_or_append_to_path_set "$dep_path" "$path_parts")"
        fi
      fi
    done
  fi

  # Finally, export the final `$PATH` environment variable
  export PATH="$path_parts"
}

# **Internal** Appends an entry to the given path array (delimited by colon
# characters) only if the entry is not already present and returns the
# resulting path array back on stdout. In so doing, this function mimicks a set
# when adding new entries. Note that any path array can be passed in, including
# ones that already contain duplicate entries.
#
# ```
# arr="a:b:c"
# arr="$(return_or_append_to_path_set "b" "$arr")"
# echo $arr
# # a:b:c
# arr="$(return_or_append_to_path_set "z" "$arr")"
# echo $arr
# # a:b:c:z
# ```
#
# Will return 0 in any case.
return_or_append_to_path_set() {
  if echo "$1" | $bb grep -q -E "(^|:)${2}(:|$)"; then
    echo "$2"
    return 0
  fi
  echo "${2}:$1"
  return 0
}

# **Internal** Sets the `$libexec_path` variable, which is the absolute path to
# the `libexec/` directory for this software.
set_libexec_path() {
  local bb
  local p
  # First check to see if we have been given a path to a `busybox` command
  if [ -n "${BUSYBOX:-}" -a -x "${BUSYBOX:-}" ]; then
    bb="$BUSYBOX"
    unset BUSYBOX
  # Next, check to see if a `busybox` command is on `PATH`
  elif command -v busybox > /dev/null; then
    bb="$(command -v busybox)"
  # Finally, check for each command required to calculate the path to libexec,
  # after which we will have a `busybox` command we can use forever after
  else
    if ! command -v basename > /dev/null; then
      exit_with "Busybox not found, so 'basename' command must be on PATH" 99
    fi
    if ! command -v dirname > /dev/null; then
      exit_with "Busybox not found, so 'dirname' command must be on PATH" 99
    fi
    if ! command -v pwd > /dev/null; then
      exit_with "Busybox not found, so 'pwd' command must be on PATH" 99
    fi
    if ! command -v readlink > /dev/null; then
      exit_with "Busybox not found, so 'readlink' command must be on PATH" 99
    fi
    bb=
  fi

  p="$($bb dirname $0)"
  p="$(cd $p; $bb pwd)/$($bb basename $0)"
  p="$($bb readlink -f $p)"
  p="$($bb dirname $p)"

  libexec_path="$($bb dirname $p)/libexec"
  return 0
}


# # Main Flow

# Set the `$libexec_path` variable containing an absolute path to `../libexec`
# from this program. This directory contains Studio type definitions and the
# `busybox` binary which is used for all shell out commands.
set_libexec_path
# Finally, unset `PATH` so there is zero chance we're going to rely on the
# operating system's commands.
unset PATH


# ## Default variables

# Absolute path to the `busybox` command
bb="$libexec_path/busybox"
# Absolute path to the `coreutils` command
cu="$libexec_path/coreutils"
# Absolute path to the `gpg` command
gpg="$libexec_path/gpg"
# Absolute path to the `jq` command
jq="$libexec_path/jq"
# The current version of this program
version='@version@'
# The author of this program
author='@author@'
# The short version of the program name which is used in logging output
program=$($bb basename $0)


# ## CLI Argument Parsing

# Parse command line flags and options.
while getopts ":vqVh" opt; do
  case $opt in
    v)
      VERBOSE=true
      QUIET=
      ;;
    q)
      QUIET=true
      VERBOSE=
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

# Now we can set up some common runtime variables that are used throughout the
# program

# The root of the filesystem. If the program is running on a seperate
# filesystem or chroot environment, this environment variable may need to be
# set.
: ${FS_ROOT:=}
# The root of the bldr tree. If `BLDR_ROOT` is set, this value is overridden,
# otherwise it defaults to `/opt/bldr`.
: ${BLDR_ROOT:=$FS_ROOT/opt/bldr}
# Location containing installed packages
BLDR_PKG_ROOT=$BLDR_ROOT/pkgs
# Location containing cached packages
BLDR_PKG_CACHE=$BLDR_ROOT/cache/pkgs
# Location containing cached gpg keys
BLDR_GPG_CACHE=$BLDR_ROOT/cache/gpg
# The default bldr package repository from where to download dependencies
: ${BLDR_REPO:=http://52.37.151.35:9632}
# Whether or not more verbose output has been requested. An unset or empty
# value means it is set to false and any other value is considered set or true.
: ${VERBOSE:=}
# Whether or not less output has been requested. An unset or empty value means
# it is set to false and any other value is considered set or true.
: ${QUIET:=}

# Next, determine the subcommand and delegate its behavior to the appropriate
# function. Note that the multiple word fragments for each case result in a
# "fuzzy matching" behavior, meaning that `studio e` is equivalent to `studio
# enter`.
case ${1:-} in
  b|bi|bin|binl|binli|binlin|binlink)
    shift
    subcommand_binlink $*
    ;;
  e|ex|exe|exec)
    shift
    subcommand_exec $*
    ;;
  i|in|ins|inst|insta|instal|install)
    shift
    subcommand_install $*
    ;;
  p|pk|pkg|pkgp|pkgpa|pkgpat|pkgpath)
    shift
    subcommand_pkgpath $*
    ;;
  v|ve|ver|vers|versi|versio|version)
    echo "$program $version"
    exit 0
    ;;
  h|he|hel|help)
    print_help
    exit 0
    ;;
  *)
    print_help
    exit_with "Invalid argument: ${1:-}" 1
    ;;
esac

# That's all, folks!
exit 0
