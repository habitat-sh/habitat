#!/bin/bash
#
# # Usage
#
# ```
# $ hab-pkg-mesosize [PKG ...]
# ```
#
# # Synopsis
#
# Create a Mesos application from a set of Habitat packages.
#

# defaults for the application
: "${CPU:="0.5"}"
: "${DISK:="0"}"
: "${INSTANCES:="1"}"
: "${MEM:="256"}"
: "${PKG:="unknown"}"

# Fail if there are any unset variables and whenever a command returns a
# non-zero exit code.
set -eu

# If the variable `$DEBUG` is set, then print the shell commands as we execute.
if [ -n "${DEBUG:-}" ]; then
  set -x
  export DEBUG
fi

# ## Help

# **Internal** Prints help
print_help() {
  echo -- "$program $version

$author

Habitat Package Mesosize - Create a Mesos application from a set of Habitat packages

USAGE:
  $program [FLAGS] [OPTIONS] <PKG_IDENT>

FLAGS:
    --help           Prints help information

OPTIONS:
    --cpu=N          CPUs for the application (float, .5 is default)
    --disk=N         Disk Space for the application (0 is default)
    --instances=N    Number of application instances to launch (1 is default)
    --mem=N          Memory for the application (MB, 256 is default)

ARGS:
    <PKG_IDENT>      Habitat package identifier (ex: acme/redis)
"
}

# **Internal** Exit the program with an error message and a status code.
#
# ```sh
# exit_with "Something bad went down" 55
# ```
exit_with() {
  if [ "${HAB_NOCOLORING:-}" = "true" ]; then
    printf -- "ERROR: %s\n" "$1"
  else
    case "${TERM:-}" in
      *term | xterm-* | rxvt | screen | screen-*)
        printf -- "\033[1;31mERROR: \033[1;37m%s\033[0m\n" "$1"
        ;;
      *)
        printf -- "ERROR: %s\n" "$1"
        ;;
    esac
  fi
  exit "$2"
}

find_system_commands() {
  if mktemp --version 2>&1 | grep -q 'GNU coreutils'; then
    _mktemp_cmd=$(command -v mktemp)
  else
    if /bin/mktemp --version 2>&1 | grep -q 'GNU coreutils'; then
      _mktemp_cmd=/bin/mktemp
    else
      exit_with "We require GNU mktemp to build Mesos applications; aborting" 1
    fi
  fi
}

# parse the CLI flags and options
parse_options() {
  for i in "$@"
  do
    case $i in
      --help)
        print_help
        exit
        ;;
      --cpu=*)
        CPU="${i#*=}"
        shift
        ;;
      --disk=*)
        DISK="${i#*=}"
        shift
        ;;
      --instances=*)
        INSTANCES="${i#*=}"
        shift
        ;;
      --mem=*)
        MEM="${i#*=}"
        shift
        ;;
      *)
        PKG=${i}
        ;;
    esac
  done
  if [ "$PKG" == "unknown" ]; then
    print_help
    exit_with "You must specify one or more Habitat packages to Mesosize." 1
  fi
}

# Create a hab studio baseimage and populate it with the application
build_tarball_image() {
  TARBALL_CONTEXT="$($_mktemp_cmd -t -d "${program}-XXXX")"
  pushd "$TARBALL_CONTEXT" > /dev/null
  env PKGS="$PKG" NO_MOUNT=1 hab studio -r "$TARBALL_CONTEXT" -t bare new
  echo "$PKG" > "$TARBALL_CONTEXT"/.hab_pkg
  popd > /dev/null
  tar -czpf "$(package_name_with_version "$PKG")".tgz -C "$TARBALL_CONTEXT" ./
}

package_name_with_version() {
  local ident_file
  ident_file=$(find "$TARBALL_CONTEXT"/"$HAB_ROOT_PATH"/pkgs/"$PKG" -name IDENT)
  awk 'BEGIN { FS = "/" }; { print $1 "-" $2 "-" $3 "-" $4 }' < "$ident_file"
}

# https://mesosphere.github.io/marathon/docs/application-basics.html
create_application_definition() {
  echo "
  {
   \"id\": \"$PKG\",
   \"cmd\": \"/bin/id -u hab &>/dev/null || /sbin/useradd hab; /bin/chown -R hab:hab *; mount -t proc proc proc/; mount -t sysfs sys sys/;mount -o bind /dev dev/; /usr/sbin/chroot . ./init.sh start $PKG\",
   \"cpus\": $CPU,
   \"disk\": $DISK,
   \"mem\": $MEM,
   \"instances\": $INSTANCES,
   \"uris\": [ \"URL_TO_$(package_name_with_version "$PKG").tgz\" ]
  }
"
  # what about exposing ports?
  }

# The root of the filesystem. If the program is running on a separate
# filesystem or chroot environment, this environment variable may need to be
# set.
: "${FS_ROOT:=}"
# The root path of the Habitat file system. If the `$HAB_ROOT_PATH` environment
# variable is set, this value is overridden, otherwise it is set to its default
: "${HAB_ROOT_PATH:=$FS_ROOT/hab}"

# The current version of Habitat Studio
version='@version@'
# The author of this program
author='@author@'
# The short version of the program name which is used in logging output
program=$(basename "$0")

find_system_commands

parse_options "$@"
build_tarball_image
# publish the tarball somewhere? upload_tarball_to_artifact_store?
create_application_definition
rm -rf "$TARBALL_CONTEXT"
