#!/bin/sh
#
# # Usage
#
# ```sh
# $ hab-studio [FLAGS] [OPTIONS] <SUBCOMMAND> [ARG ...]
# ```
#
# See the `print_help()` function below for complete usage instructions.
#
# # Synopsis
#
# blah
#
# # License and Copyright
#
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

# TESTING CHANGES
# Documentation on testing local changes to this file and its friends in
# ../libexec lives here:
# https://github.com/habitat-sh/habitat/blob/master/BUILDING.md#testing-changes

# # Internals

# Fail if there are any unset variables and whenever a command returns a
# non-zero exit code.
set -eu

# If the variable `$DEBUG` is set, then print the shell commands as we execute.
if [ -n "${DEBUG:-}" ]; then
  set -x
  export DEBUG
fi

# **Internal** Set the verbose flag (i.e. `-v`) for any coreutils-like commands
# if verbose mode was requested. This definition has to occur before the usage
# of $v without quotes in the file to not trip the linter. If quotes are added
# around $v, it causes failures in non-verbose mode. For example:
#   $bb mkdir -p "$v" "$HAB_STUDIO_ROOT"/dev
# Results in: mkdir: can't create directory '': No such file or directory
set_v_flag() {
  if [ -n "${VERBOSE:-}" ]; then
    v="-v"
  else
    v=
  fi
}

# ## Help/Usage functions

# **Internal** Prints help and usage information. Straight forward, no?
print_help() {
  echo "$program $version

$author

Habitat Studios - Plan for success!

USAGE:
        $program [FLAGS] [OPTIONS] <SUBCOMMAND> [ARG ..]

COMMON FLAGS:
    -h  Prints this message
    -n  Do not mount the source path into the Studio (default: mount the path)
    -N  Do not mount the source artifact cache path into the Studio (default: mount the path)
    -q  Prints less output for better use in scripts
    -v  Prints more verbose output
    -V  Prints version information
    -D  Use a Docker Studio instead of a chroot Studio

COMMON OPTIONS:
    -a <ARTIFACT_PATH>    Sets the source artifact cache path (default: /hab/cache/artifacts)
    -k <HAB_ORIGIN_KEYS>  Installs secret origin keys (default:\$HAB_ORIGIN )
    -r <HAB_STUDIO_ROOT>  Sets a Studio root (default: /hab/studios/<DIR_NAME>)
    -s <SRC_PATH>         Sets the source path (default: \$PWD)
    -t <STUDIO_TYPE>      Sets a Studio type when creating (default: default)
                          Valid types: [default baseimage busybox stage1]

SUBCOMMANDS:
    build     Build using a Studio
    enter     Interactively enter a Studio
    help      Prints this message
    new       Creates a new Studio
    rm        Destroys a Studio
    run       Run a command in a Studio
    version   Prints version information

ENVIRONMENT VARIABLES:
    ARTIFACT_PATH          Sets the source artifact cache path (\`-a' option overrides)
    HAB_NOCOLORING         Disables text coloring mode despite TERM capabilities
    HAB_NONINTERACTIVE     Disables interactive progress bars despite tty
    HAB_ORIGIN             Propagates this variable into any studios
    HAB_ORIGIN_KEYS        Installs secret keys (\`-k' option overrides)
    HAB_STUDIOS_HOME       Sets a home path for all Studios (default: /hab/studios)
    HAB_STUDIO_NOSTUDIORC  Disables sourcing a \`.studiorc' in \`studio enter'
    HAB_STUDIO_ROOT        Sets a Studio root (\`-r' option overrides)
    HAB_STUDIO_SUP         Sets args for a Supervisor in \`studio enter'
    NO_ARTIFACT_PATH       If set, do not mount the source artifact cache path (\`-N' flag overrides)
    NO_SRC_PATH            If set, do not mount the source path (\`-n' flag overrides)
    QUIET                  Prints less output (\`-q' flag overrides)
    SRC_PATH               Sets the source path (\`-s' option overrides)
    STUDIO_TYPE            Sets a Studio type when creating (\`-t' option overrides)
    VERBOSE                Prints more verbose output (\`-v' flag overrides)
    http_proxy             Sets an http_proxy environment variable inside the Studio
    https_proxy            Sets an https_proxy environment variable inside the Studio
    no_proxy               Sets a no_proxy environment variable inside the Studio

SUBCOMMAND HELP:
    $program <SUBCOMMAND> -h

EXAMPLES:

    # Create a new default Studio
    $program new

    # Enter the default Studio
    $program enter

    # Run a command in the default Studio
    $program run wget --version

    # Destroy the default Studio
    $program rm

    # Create and enter a busybox type Studio with a custom root
    $program -r /opt/slim -t busybox enter

    # Run a command in the slim Studio, showing only the command output
    $program -q -r /opt/slim run busybox ls -l /

    # Verbosely destroy the slim Studio
    $program -v -r /opt/slim rm
"
}

print_build_help() {
  echo "${program}-build $version

$author

Habitat Studios - execute a build using a Studio

USAGE:
        $program [COMMON_FLAGS] [COMMON_OPTIONS] build [FLAGS] [PLAN_DIR]

FLAGS:
    -R  Reuse a previous Studio state (default: clean up before building)

EXAMPLES:

    # Build a Redis plan
    $program build plans/redis

    # Reuse previous Studio for a build
    $program build -R plans/glibc
"
}

print_enter_help() {
  echo "${program}-enter $version

$author

Habitat Studios - interactively enter a Studio

USAGE:
        $program [COMMON_FLAGS] [COMMON_OPTIONS] enter
"
}

print_new_help() {
  echo "${program}-new $version

$author

Habitat Studios - create a new Studio

USAGE:
        $program [COMMON_FLAGS] [COMMON_OPTIONS] new
"
}

print_rm_help() {
  echo "${program}-rm $version

$author

Habitat Studios - destroy a Studio

USAGE:
        $program [COMMON_FLAGS] [COMMON_OPTIONS] rm
"
}

print_run_help() {
  echo "${program}-run $version

$author

Habitat Studios - run a command in a Studio

USAGE:
        $program [COMMON_FLAGS] [COMMON_OPTIONS] run [CMD] [ARG ..]

CMD:
    Command to run in the Studio

ARG:
    Arguments to the command

EXAMPLES:

    $program run wget --version
"
}


# ## Subcommand functions
#
# These are the implementations for each subcommand in the program.

# **Internal** Parses options and flags for `new` subcommand.
subcommand_new() {
  OPTIND=1
  # Parse command line flags and options
  while getopts ":h" opt; do
    case $opt in
      h)
        print_new_help
        exit 0
        ;;
      \?)
        print_new_help
        exit_with "Invalid option:  -$OPTARG" 1
        ;;
    esac
  done
  # Shift off all parsed token in `$*` so that the subcommand is now `$1`.
  shift "$((OPTIND - 1))"

  trap cleanup_studio EXIT

  new_studio
}

# **Internal** Parses options and flags for `rm` subcommand.
subcommand_rm() {
  OPTIND=1
  # Parse command line flags and options
  while getopts ":h" opt; do
    case $opt in
      h)
        print_rm_help
        exit 0
        ;;
      \?)
        print_rm_help
        exit_with "Invalid option:  -$OPTARG" 1
        ;;
    esac
  done
  # Shift off all parsed token in `$*` so that the subcommand is now `$1`.
  shift "$((OPTIND - 1))"

  rm_studio "$@"
}

# **Internal** Parses options and flags for `enter` subcommand.
subcommand_enter() {
  OPTIND=1
  # Parse command line flags and options
  while getopts ":h" opt; do
    case $opt in
      h)
        print_enter_help
        exit 0
        ;;
      \?)
        print_enter_help
        exit_with "Invalid option:  -$OPTARG" 1
        ;;
    esac
  done
  # Shift off all parsed token in `$*` so that the subcommand is now `$1`.
  shift "$((OPTIND - 1))"

  trap cleanup_studio EXIT

  new_studio
  enter_studio "$@"
}

# **Internal** Parses options and flags for `build` subcommand.
subcommand_build() {
  OPTIND=1
  # Parse command line flags and options
  while getopts ":hR" opt; do
    case $opt in
      h)
        print_build_help
        exit 0
        ;;
      R)
        reuse=true
        ;;
      \?)
        print_build_help
        exit_with "Invalid option:  -$OPTARG" 1
        ;;
    esac
  done
  # Shift off all parsed token in `$*` so that the subcommand is now `$1`.
  shift "$((OPTIND - 1))"

  trap cleanup_studio EXIT

  if [ -z "${reuse:-}" ]; then
    _STUDIO_TYPE="$STUDIO_TYPE"
    rm_studio
    STUDIO_TYPE="$_STUDIO_TYPE"
    unset _STUDIO_TYPE
  fi
  new_studio
  build_studio "$@"
}

# **Internal** Parses options and flags for `run` subcommand.
subcommand_run() {
  OPTIND=1
  # Parse command line flags and options
  while getopts ":h" opt; do
    case $opt in
      h)
        print_run_help
        exit 0
        ;;
      \?)
        print_run_help
        exit_with "Invalid option:  -$OPTARG" 1
        ;;
    esac
  done
  # Shift off all parsed token in `$*` so that the subcommand is now `$1`.
  shift "$((OPTIND - 1))"

  trap cleanup_studio EXIT

  new_studio
  run_studio "$@"
}


# **Internal**  If a non-zero sized Studio configuration is not found, exit the program.
exit_if_no_studio_config() {
  if [ ! -s "$HAB_STUDIO_ROOT/.studio" ]; then
    exit_with "Directory $HAB_STUDIO_ROOT does not appear to be a Studio, aborting" 5
  fi
}

# **Internal** Check if a pre-existing Studio configuration is found and use
# that to determine the type and assign it to STUDIO_TYPE
source_studio_type_config() {
  if [ -s "$studio_config" ]; then
    # shellcheck disable=1090
    . "$studio_config"
    STUDIO_TYPE=${studio_type:?}
  fi
}

# **Internal** Creates a new Studio.
new_studio() {
  source_studio_type_config

  # Validate the type specified is valid and set a default if unset
  case "${STUDIO_TYPE:-unset}" in
    unset|default)
      # Set the default/unset type
      STUDIO_TYPE=default
      ;;
    busybox|stage1|baseimage|bare)
      # Confirmed valid types
      ;;
    *)
      # Everything else is invalid
      exit_with "Invalid Studio type: $STUDIO_TYPE" 2
      ;;
  esac

  # Properly canonicalize the root path of the Studio by following all symlinks.
  $bb mkdir -p "$HAB_STUDIO_ROOT"
  HAB_STUDIO_ROOT="$($bb readlink -f "$HAB_STUDIO_ROOT")"

  info "Creating Studio at $HAB_STUDIO_ROOT ($STUDIO_TYPE)"

  # Mount filesystems

  $bb mkdir -p $v "$HAB_STUDIO_ROOT"/dev
  $bb mkdir -p $v "$HAB_STUDIO_ROOT"/proc
  $bb mkdir -p $v "$HAB_STUDIO_ROOT"/sys
  $bb mkdir -p $v "$HAB_STUDIO_ROOT"/run
  $bb mkdir -p $v "$HAB_STUDIO_ROOT"/var/run

  # Unless `$NO_MOUNT` is set, mount filesystems such as `/dev`, `/proc`, and
  # company. If the mount already exists, skip it to be all idempotent and
  # nerdy like that
  if [ -z "${NO_MOUNT}" ]; then
    if ! $bb mount | $bb grep -q "on $HAB_STUDIO_ROOT/dev type"; then
      if [ -z "${KRANGSCHNAK+x}" ]; then
        $bb mount $v --bind /dev "$HAB_STUDIO_ROOT"/dev
      else
        $bb mount $v --rbind /dev "$HAB_STUDIO_ROOT"/dev
      fi
    fi

    if ! $bb mount | $bb grep -q "on $HAB_STUDIO_ROOT/dev/pts type"; then
      $bb mount $v -t devpts devpts "$HAB_STUDIO_ROOT"/dev/pts -o gid=5,mode=620
    fi
    if ! $bb mount | $bb grep -q "on $HAB_STUDIO_ROOT/proc type"; then
      $bb mount $v -t proc proc "$HAB_STUDIO_ROOT"/proc
    fi
    if ! $bb mount | $bb grep -q "on $HAB_STUDIO_ROOT/sys type"; then
      if [ -z "${KRANGSCHNAK+x}" ]; then
        $bb mount $v -t sysfs sysfs "$HAB_STUDIO_ROOT"/sys
      else
        $bb mount $v --rbind /sys "$HAB_STUDIO_ROOT"/sys
      fi
    fi
    if ! $bb mount | $bb grep -q "on $HAB_STUDIO_ROOT/run type"; then
      $bb mount $v -t tmpfs tmpfs "$HAB_STUDIO_ROOT"/run
    fi
    if [ -e /var/run/docker.sock ]; then
      if ! $bb mount | $bb grep -q "on $HAB_STUDIO_ROOT/var/run/docker.sock type"; then
        $bb touch "$HAB_STUDIO_ROOT"/var/run/docker.sock
        $bb mount $v --bind /var/run/docker.sock "$HAB_STUDIO_ROOT"/var/run/docker.sock
      fi
    fi

    if [ -h "$HAB_STUDIO_ROOT/dev/shm" ]; then
      $bb mkdir -p $v "$HAB_STUDIO_ROOT/$($bb readlink "$HAB_STUDIO_ROOT"/dev/shm)"
    fi

    # Mount the `$ARTIFACT_PATH` under `/hab/cache/artifacts` in the Studio,
    # unless `$NO_ARTIFACT_PATH` are set
    if [ -z "${NO_ARTIFACT_PATH}" ]; then
      studio_artifact_path="${HAB_STUDIO_ROOT}${HAB_CACHE_ARTIFACT_PATH}"
      if ! $bb mount | $bb grep -q "on $studio_artifact_path type"; then
        $bb mkdir -p $v "$ARTIFACT_PATH"
        $bb mkdir -p $v "$studio_artifact_path"
        $bb mount $v --bind "$ARTIFACT_PATH" "$studio_artifact_path"
      fi
    fi
  fi

  # Create root filesystem

  for top_level_dir in bin etc home lib mnt opt sbin var; do
    $bb mkdir -p $v "$HAB_STUDIO_ROOT/$top_level_dir"
  done

  $bb install -d $v -m 0750 "$HAB_STUDIO_ROOT/root"
  $bb install -d $v -m 1777 "$HAB_STUDIO_ROOT/tmp" "$HAB_STUDIO_ROOT/var/tmp"

  for usr_dir in bin include lib libexec sbin; do
    $bb mkdir -p $v "$HAB_STUDIO_ROOT/usr/$usr_dir"
  done

  for usr_share_dir in doc info locale man misc terminfo zoneinfo; do
    $bb mkdir -p $v "$HAB_STUDIO_ROOT/usr/share/$usr_share_dir"
  done

  for usr_share_man_dir_num in 1 2 3 4 5 6 7 8; do
    $bb mkdir -p $v "$HAB_STUDIO_ROOT/usr/share/man/man$usr_share_man_dir_num"
  done
  # If the system is 64-bit, a few symlinks will be required
  case $($bb uname -m) in
  x86_64)
    $bb ln -sf $v lib "$HAB_STUDIO_ROOT/lib64"
    $bb ln -sf $v lib "$HAB_STUDIO_ROOT/usr/lib64"
    ;;
  esac

  for var_dir in log mail spool opt cache local; do
    $bb mkdir -p $v "$HAB_STUDIO_ROOT/var/$var_dir"
  done

  $bb ln -sf $v /run/lock "$HAB_STUDIO_ROOT/var/lock"

  $bb mkdir -p $v "$HAB_STUDIO_ROOT/var/lib/color"
  $bb mkdir -p $v "$HAB_STUDIO_ROOT/var/lib/misc"
  $bb mkdir -p $v "$HAB_STUDIO_ROOT/var/lib/locate"

  $bb ln -sf $v /proc/self/mounts "$HAB_STUDIO_ROOT/etc/mtab"

  # Load the appropriate type strategy to complete the setup
  if [ -n "${HAB_STUDIO_BINARY:-}" ]; then
    studio_type_dir="$studio_binary_libexec_path"
  else
    studio_type_dir="$libexec_path"
  fi
  # shellcheck disable=1090
  . "$studio_type_dir/hab-studio-type-${STUDIO_TYPE}.sh"

  # These two are needed to satisfy some software builds.
  #
  # They're also not overwritten, because we actually *add* entries to
  # them later on in the process. Since you can re-use studios, that
  # would wipe out those changes.
  #
  # TODO (CM): we should consolidate this stuff.
  copy_minimal_default_file_if_not_present "/etc/passwd"
  copy_minimal_default_file_if_not_present "/etc/group"

  # This one is nice for interactive work.
  #
  # Though we don't do anything else with this file, it's conceivable
  # that users might modify it and want those changes to persist in a
  # long-lived studio.
  copy_minimal_default_file_if_not_present "/etc/inputrc"

  # Copy minimal networking and DNS resolution configuration files into the
  # Studio filesystem so that commands such as `wget(1)` will work.
  #
  # TODO (CM): Unsure why we unconditionally copy this file, but
  # not the two files below.
  copy_minimal_default_file "/etc/nsswitch.conf"
  for f in /etc/hosts /etc/resolv.conf; do
    # Note: These files are copied **from the host**
    $bb cp $v $f "$HAB_STUDIO_ROOT$f"
  done

  # Invoke the type's implementation
  finish_setup

  # Add a Studio configuration file at the root of the filesystem
  $bb cat <<EOF > "$studio_config"
studio_type="$studio_type"
studio_path="$studio_path"
studio_env_command="${studio_env_command:?}"
studio_enter_environment="${studio_enter_environment?}"
studio_enter_command="${studio_enter_command:?}"
studio_build_environment="${studio_build_environment?}"
studio_build_command="${studio_build_command?}"
studio_run_environment="${studio_run_environment?}"
EOF

  # If `/etc/profile` is not present, create a minimal version with convenient
  # helper functions. "bare" studio doesn't need an /etc/profile
  if [ "$STUDIO_TYPE" != "bare" ]; then
    pfile="$HAB_STUDIO_ROOT/etc/profile"
    if [ ! -f "$pfile" ] || ! $bb grep -q '^record() {$' "$pfile"; then
      if [ -n "$VERBOSE" ]; then
        echo "> Creating /etc/profile"
      fi

      if [ -n "${HAB_STUDIO_BINARY:-}" ]; then
        studio_profile_dir="$studio_binary_libexec_path"
      else
        studio_profile_dir="$libexec_path"
      fi
      $bb cat "$studio_profile_dir/hab-studio-profile.sh" >> "$pfile"

    fi

    $bb mkdir -p $v "$HAB_STUDIO_ROOT/src"
    # Mount the `$SRC_PATH` under `/src` in the Studio, unless either `$NO_MOUNT`
    # or `$NO_SRC_PATH` are set
    if [ -z "${NO_MOUNT}" ] && [ -z "${NO_SRC_PATH}" ]; then
      if ! $bb mount | $bb grep -q "on $HAB_STUDIO_ROOT/src type"; then
        $bb mount $v --bind "$SRC_PATH" "$HAB_STUDIO_ROOT/src"
      fi
    fi
  fi
}

# **Internal** Interactively enter a Studio.
enter_studio() {
  exit_if_no_studio_config
  source_studio_type_config

  env="$(chroot_env "$studio_path" "$studio_enter_environment")"
  info "Entering Studio at $HAB_STUDIO_ROOT ($STUDIO_TYPE)"
  report_env_vars
  echo

  if [ -n "$VERBOSE" ]; then
    set -x
  fi

  # Become the `chroot` process
  # Note: env and studio_enter_command must NOT be quoted
  # shellcheck disable=2086
  $bb chroot "$HAB_STUDIO_ROOT" "$studio_env_command" -i $env $studio_enter_command "$@"
}

# **Internal** Run a build command using a Studio.
build_studio() {
  exit_if_no_studio_config
  source_studio_type_config

  # If a build command is not set, then this type does not support the `build`
  # subcommand and should abort.
  if [ -z "$studio_build_command" ]; then
    exit_with "Studio at $HAB_STUDIO_ROOT ($STUDIO_TYPE) does not support 'build'" 10
  fi

  env="$(chroot_env "$studio_path" "$studio_build_environment")"

  info "Building '$*' in Studio at $HAB_STUDIO_ROOT ($STUDIO_TYPE)"
  report_env_vars

  if [ -n "$VERBOSE" ]; then
    set -x
  fi

  # Run the build command in the `chroot` environment
  # Note: studio_run_command, env and studio_run_command must NOT be quoted
  # shellcheck disable=2086
  echo $studio_build_command "$@" | $bb chroot "$HAB_STUDIO_ROOT" "$studio_env_command" -i $env ${studio_run_command:?}
}

# **Internal** Run an arbitrary command in a Studio.
run_studio() {
  exit_if_no_studio_config
  source_studio_type_config

  env="$(chroot_env "$studio_path" "$studio_run_environment")"

  info "Running '$*' in Studio at $HAB_STUDIO_ROOT ($STUDIO_TYPE)"

  if [ -n "$VERBOSE" ]; then
    set -x
  fi

  # Run the command in the `chroot` environment
  # Note: env and studio_run_command must NOT be quoted
  # shellcheck disable=2086
  echo "$@" | $bb chroot "$HAB_STUDIO_ROOT" "$studio_env_command" -i $env $studio_run_command
}

# **Internal** Destroy a Studio.
rm_studio() {

  source_studio_type_config

  if [ -d "$HAB_STUDIO_ROOT" ]; then
    # Properly canonicalize the root path of the Studio by following all symlinks.
    HAB_STUDIO_ROOT="$($bb readlink -f "$HAB_STUDIO_ROOT")"
  fi

  info "Destroying Studio at $HAB_STUDIO_ROOT ($STUDIO_TYPE)"

  cleanup_studio

  if $bb mount | $bb grep -q "on ${HAB_STUDIO_ROOT}.* type"; then
    # There are still mounted filesystems under the root. In the spirit of "do
    # no further harm", we abort here with a message so the user can resolve
    # and retry.
    >&2 echo "After unmounting all known filesystems, there are still \
remaining mounted filesystems under ${HAB_STUDIO_ROOT}:"
    $bb mount \
      | $bb grep "on ${HAB_STUDIO_ROOT}.* type" \
      | $bb sed 's#^#    * #'
    >&2 echo "Unmount these remaining filesystem using \`umount(8)'and retry \
the last command."
    exit_with "Remaining mounted filesystems found under $HAB_STUDIO_ROOT" \
      "$ERR_REMAINING_MOUNTS"
  else
    # No remaining mounted filesystems, so remove remaining files and dirs
    $bb rm -rf $v "$HAB_STUDIO_ROOT"
  fi
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

  if [ "${HAB_NOCOLORING:-}" = "true" ]; then
    echo "   ${program:-unknown}: $1"
  else
    case "${TERM:-}" in
      *term | xterm-* | rxvt | screen | screen-*)
        printf -- "   \033[1;36m%s: \033[1;37m%s\033[0m\n" "${program:-unknown}" "$1"
        ;;
      *)
        echo "   ${program:-unknown}: $1"
        ;;
    esac
  fi
  return 0
}

# **Internal** Exit if current user is not root.
ensure_root() {
  # Early return if we are root, yay!
  if [ "$($bb id -u)" -eq 0 ]; then
    return
  fi

  # Otherwise, prepare to die with message formatting similar to the `hab` program.
  warn_msg="Running Habitat Studio requires root or administrator privileges. \
Please retry this command as a super user or use a privilege-granting facility such as sudo."
  fatal_msg="Root or administrator permissions required to complete operation"

  if [ "${HAB_NOCOLORING:-}" = "true" ]; then
    printf -- "∅ %s\n\n✗✗✗\n✗✗✗ %s\n✗✗✗\n" "$warn_msg" "$fatal_msg"
  else
    case "${TERM:-}" in
      *term | xterm-* | rxvt | screen | screen-*)
        printf -- "\033[0;33m∅ %s\033[0m\n\n\033[0;31m✗✗✗\n✗✗✗ %s\n✗✗✗\033[0m\n" "$warn_msg" "$fatal_msg"
        ;;
      *)
        printf -- "∅ %s\n\n✗✗✗\n✗✗✗ %s\n✗✗✗\n" "$warn_msg" "$fatal_msg"
        ;;
    esac
  fi
  exit 9
}

# **Internal** Exit the program with an error message and a status code.
#
# ```sh
# exit_with "Something bad went down" 55
# ```
exit_with() {
  if [ "${HAB_NOCOLORING:-}" = "true" ]; then
    echo "ERROR: $1"
  else
    case "${TERM:-}" in
      *term | xterm-* | rxvt | screen | screen-*)
        printf -- "\033[1;31mERROR: \033[1;37m%s\033[0m\n" "$1"
        ;;
      *)
        echo "ERROR: $1"
        ;;
    esac
  fi
  exit "$2"
}

# **Internal** Removes any potential malicious secrets
sanitize_secrets() {
  for x in HAB_BINLINK_DIR HAB_ORIGIN HOME LC_ALL PATH PWD STUDIO_TYPE TERM TERMINFO; do
    unset "HAB_STUDIO_SECRET_$x"
  done
}

# **Internal** Builds up a secret environment based on the prefix `HAB_STUDIO_SECRET_`
# to pass into the studio
load_secrets() {
  sanitize_secrets
  $bb env | $bb awk -F '=' '/^HAB_STUDIO_SECRET_/ {gsub(/HAB_STUDIO_SECRET_/, ""); print}'
}

# **Internal** Builds up the environment set to pass to an `env(1)` command for
# use in a `chroot` environment which is printed on stdout.
chroot_env() {
  studio_path="$1"
  extra_env="$2"

  # Set the environment which will be passed to `env(1)` to initialize the
  # session.
  env="LC_ALL=POSIX HOME=/root TERM=${TERM:-} PATH=$studio_path"
  # Add `STUDIO_TYPE` to the environment
  env="$env STUDIO_TYPE=$STUDIO_TYPE"
  # Add any additional environment variables from the Studio config, based on
  # type
  if [ -n "$extra_env" ]; then
    env="$env $extra_env"
  fi
  # If a Habitat config filetype ignore string is set, then propagate it
  # into the Studio's environment.
  if [ -n "${HAB_CONFIG_EXCLUDE:-}" ]; then
    env="$env HAB_CONFIG_EXCLUDE=$HAB_CONFIG_EXCLUDE"
  fi
  # If a Habitat Auth Token is set, then propagate it into the Studio's
  # environment.
  if [ -n "${HAB_AUTH_TOKEN:-}" ]; then
    env="$env HAB_AUTH_TOKEN=$HAB_AUTH_TOKEN"
  fi
  # If a Habitat Builder URL is set, then propagate it into the Studio's
  # environment.
  if [ -n "${HAB_BLDR_URL:-}" ]; then
    env="$env HAB_BLDR_URL=$HAB_BLDR_URL"
  fi
  # If a Habitat Depot Channel is set, then propagate it into the Studio's
  # environment.
  if [ -n "${HAB_BLDR_CHANNEL:-}" ]; then
    env="$env HAB_BLDR_CHANNEL=$HAB_BLDR_CHANNEL"
  fi
  # If a no coloring environment variable is set, then propagate it into the Studio's
  # environment.
  if [ -n "${HAB_NOCOLORING:-}" ]; then
    env="$env HAB_NOCOLORING=$HAB_NOCOLORING"
  fi
  # If a noninteractive environment variable is set, then propagate it into the Studio's
  # environment.
  if [ -n "${HAB_NONINTERACTIVE:-}" ]; then
    env="$env HAB_NONINTERACTIVE=$HAB_NONINTERACTIVE"
  fi
  # If a Habitat origin name is set, then propagate it into the Studio's
  # environment.
  if [ -n "${HAB_ORIGIN:-}" ]; then
    env="$env HAB_ORIGIN=$HAB_ORIGIN"
  fi
  # If a skip .studiorc environment variable is set, then propagate it into the
  # Studio's environment.
  if [ -n "${HAB_STUDIO_NOSTUDIORC:-}" ]; then
    env="$env HAB_STUDIO_NOSTUDIORC=$HAB_STUDIO_NOSTUDIORC"
  fi
  # Used to customize the arguments to pass to an automatically launched
  # Supervisor, or to disable the automatic launching (by setting to 'false').
  if [ -n "${HAB_STUDIO_SUP:-}" ]; then
    # We want to pass a value that contains spaces, so we'll encode the spaces
    # for unpacking inside the Studio. Sorry world, but it's after 11pm.
    env="$env HAB_STUDIO_SUP=$(echo "$HAB_STUDIO_SUP" | $bb sed 's/ /__sp__/g')"
  fi
  # If a Habitat update strategy frequency is set, then propagate it into the
  # Studio's environment.
  if [ -n "${HAB_UPDATE_STRATEGY_FREQUENCY_MS:-}" ]; then
    env="$env HAB_UPDATE_STRATEGY_FREQUENCY_MS=$HAB_UPDATE_STRATEGY_FREQUENCY_MS"
  fi
  # If a Habitat studio binary is set, then propagate it into the Studio's
  # environment.
  if [ -n "${HAB_STUDIO_BINARY:-}" ]; then
    env="$env HAB_STUDIO_BINARY=$HAB_STUDIO_BINARY"
  fi
  # If DO_CHECK is set, then propagate it into the Studio's environment.
  if [ -n "${DO_CHECK:-}" ]; then
    env="$env DO_CHECK=$DO_CHECK"
  fi

  # If HTTP proxy variables are detected in the current environment, propagate
  # them into the Studio's environment.
  if [ -n "${http_proxy:-}" ]; then
    env="$env http_proxy=$http_proxy"
  fi
  if [ -n "${https_proxy:-}" ]; then
    env="$env https_proxy=$https_proxy"
  fi
  if [ -n "${no_proxy:-}" ]; then
    # If you pass whitespace here, bash will loose its mind when we do expansion
    # in the exec later on. To spare you, and me, and everyone else, we go ahead
    # and take care of that little whitespace problem for you.
    #
    # Thanks, Docker, for passing unnecessary spaces. You're a peach.
    env="$env no_proxy=$(echo "$no_proxy" | $bb sed 's/, /,/g')"
  fi

  env="$env $(load_secrets)"

  echo "$env"
  return 0
}

# **Internal** Prints out any important environment variables that will be used
# inside the Studio.
report_env_vars() {
  if [ -n "${HAB_CONFIG_EXCLUDE:-}" ]; then
    info "Exported: HAB_CONFIG_EXCLUDE=$HAB_CONFIG_EXCLUDE"
  fi
  if [ -n "${HAB_AUTH_TOKEN:-}" ]; then
    info "Exported: HAB_AUTH_TOKEN=[redacted]"
  fi
  if [ -n "${HAB_ORIGIN:-}" ]; then
    info "Exported: HAB_ORIGIN=$HAB_ORIGIN"
  fi
  if [ -n "${HAB_BLDR_URL:-}" ]; then
    info "Exported: HAB_BLDR_URL=$HAB_BLDR_URL"
  fi
  if [ -n "${HAB_BLDR_CHANNEL:-}" ]; then
    info "Exported: HAB_BLDR_CHANNEL=$HAB_BLDR_CHANNEL"
  fi
  if [ -n "${HAB_NOCOLORING:-}" ]; then
    info "Exported: HAB_NOCOLORING=$HAB_NOCOLORING"
  fi
  if [ -n "${HAB_NONINTERACTIVE:-}" ]; then
    info "Exported: HAB_NONINTERACTIVE=$HAB_NONINTERACTIVE"
  fi
  if [ -n "${HAB_STUDIO_NOSTUDIORC:-}" ]; then
    info "Exported: HAB_STUDIO_NOSTUDIORC=$HAB_STUDIO_NOSTUDIORC"
  fi
  if [ -n "${HAB_STUDIO_SUP:-}" ]; then
    info "Exported: HAB_STUDIO_SUP=$HAB_STUDIO_SUP"
  fi
  if [ -n "${HAB_UPDATE_STRATEGY_FREQUENCY_MS:-}" ]; then
    info "Exported: HAB_UPDATE_STRATEGY_FREQUENCY_MS=$HAB_UPDATE_STRATEGY_FREQUENCY_MS"
  fi
  if [ -n "${http_proxy:-}" ]; then
    info "Exported: http_proxy=$http_proxy"
  fi
  if [ -n "${https_proxy:-}" ]; then
    info "Exported: https_proxy=$https_proxy"
  fi
  if [ -n "${no_proxy:-}" ]; then
    info "Exported: no_proxy=$no_proxy"
  fi

  for secret_name in $(load_secrets | $bb cut -d = -f 1); do
    info "Exported: $secret_name=[redacted]"
  done
}

# **Internal** Run when an interactive studio exits.
cleanup_studio() {
  kill_launcher
  chown_artifacts
  unmount_filesystems
}

# **Internal**  Run a command which may fail without aborting whole script
try() {
  if "$@"; then
    status=$?
  else
    status=$?
    >&2 echo "Warning: '$*' failed with status $status"
  fi
}

# **Internal** Kills a Launcher process, if one exists.
kill_launcher() {
  pid_file="$HAB_STUDIO_ROOT/hab/sup/default/LOCK"

  if [ -f "$pid_file" ]; then
    try "$bb" kill "$($bb cat "$pid_file")" && try "$bb" rm -f "$pid_file"
  fi
}

# **Internal** Updates file ownership on files under the artifact cache path
# using the ownership of the artifact cache directory to determine the target
# uid and gid. This is done in an effort to leave files residing in a user
# directory not to be owned by root.
chown_artifacts() {
  if [ -z "${NO_MOUNT}" ] \
  && [ -z "${NO_ARTIFACT_PATH}" ] \
  && [ -d "$ARTIFACT_PATH" ]; then
    artifact_path_owner="$(try "$bb" stat -c '%u:%g' "$ARTIFACT_PATH")"
    try "$bb" chown -R "$artifact_path_owner" "$ARTIFACT_PATH"
  fi
}

# **Internal** Unmount mount point if mounted and abort if an unmount is
# unsuccessful.
#
# ARGS: [umount_options] <mount_point>
umount_fs() {
  eval _mount_point=\$$# # getting the last arg is surprisingly hard

  if is_fs_mounted "${_mount_point:?}"; then
    # Filesystem is mounted, so attempt to unmount
    if $bb umount "$@"; then
      # `umount` command was successful
      if ! is_fs_mounted "$_mount_point"; then
        # Filesystem is confirmed umounted, return success
        return 0
      else
        # Despite a successful umount, filesystem is still mounted
        #
        # TODO fn: there may a race condition here: if the `umount` is
        # performed asynchronously then it might still be reported as mounted
        # when the umounting is still queued up. We're erring on the side of
        # catching any possible races here to determine if there's a problem or
        # not. If this unduly impacts user experience then an alternate
        # approach is to wait/poll until the filesystem is unmounted (with a
        # deadline to abort).
        >&2 echo "After unmounting filesystem '$_mount_point', the mount \
persisted. Check that the filesystem is no longer in the mounted using \
\`mount(8)'and retry the last command."
        exit_with "Mount of $_mount_point persists" "$ERR_MOUNT_PERSISTS"
      fi
    else
      # `umount` command reported a failure
      >&2 echo "An error occurred when unmounting filesystem '$_mount_point'"
      exit_with "Unmount of $_mount_point failed" "$ERR_UMOUNT_FAILED"
    fi
  else
    # Filesystem is not mounted, return success
    return 0
  fi
}

# **Internal** Determines if a given filesystem is currently mounted. Returns 0
# if true and non-zero otherwise.
is_fs_mounted() {
  _mount_point="${1:?}"

  $bb mount | $bb grep -q "on $_mount_point type"
}

# **Internal** Unmounts file system mounts if mounted. The order of file system
# unmounting is important as it is the opposite of the initial mount order.
#
# Any failures to successfully unmount a filesystem that is mounted will result
# in the program aborting with an error message. As this function's behavior is
# convergent on success and fast fail on failures, this can be safely run
# multiple times across differnt program invocations.
unmount_filesystems() {
  umount_fs $v -l "$HAB_STUDIO_ROOT/src"

  studio_artifact_path="${HAB_STUDIO_ROOT}${HAB_CACHE_ARTIFACT_PATH}"
  umount_fs $v -l "$studio_artifact_path"

  umount_fs $v "$HAB_STUDIO_ROOT/run"

  if [ -z "${KRANGSCHNAK+x}" ]; then
    umount_fs $v "$HAB_STUDIO_ROOT/sys"
  else
    umount_fs $v -l "$HAB_STUDIO_ROOT/sys"
  fi

  umount_fs $v "$HAB_STUDIO_ROOT/proc"

  umount_fs $v "$HAB_STUDIO_ROOT/dev/pts"

  umount_fs $v -l "$HAB_STUDIO_ROOT/dev"

  umount_fs $v -l "$HAB_STUDIO_ROOT/var/run/docker.sock"
}

# **Internal** Sets the `$libexec_path` variable, which is the absolute path to
# the `libexec/` directory for this software.
set_libexec_path() {
  # First check to see if we have been given a path to a `busybox` command
  if [ -n "${BUSYBOX:-}" ] && [ -x "${BUSYBOX:-}" ]; then
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

  if [ -n "${HAB_STUDIO_BINARY:-}" ]; then
    version="$(unset HAB_STUDIO_BINARY; hab studio version | $bb cut -d ' ' -f 2)"
    libexec_path="$(unset HAB_STUDIO_BINARY; hab pkg path core/hab-studio)/libexec"
    studio_binary_libexec_path="$($bb dirname "$HAB_STUDIO_BINARY")/../libexec"
  else
    p=$($bb dirname "$0")
    p=$(cd "$p"; $bb pwd)/$($bb basename "$0")
    p=$($bb readlink -f "$p")
    p=$($bb dirname "$p")

    libexec_path="$($bb dirname "$p")/libexec"
  fi
  return 0
}

# If `file_path` is not present in the studio, copy in a minimal
# default version from the studio package's `defaults` directory.
copy_minimal_default_file_if_not_present() {
    file_path="${1}"
    if [ -f "${HAB_STUDIO_ROOT}${file_path}" ]; then
        if [ -n "$VERBOSE" ]; then
            echo "> Skipping creation of ${file_path}; file exists"
        fi
    else
        copy_minimal_default_file "${file_path}"
    fi
}

copy_minimal_default_file() {
    file_path="${1}"
    defaults_path="$($bb dirname "${libexec_path}")/defaults"
    if [ -n "$VERBOSE" ]; then
        echo "> Creating minimal ${file_path}"
    fi
    if [ -f "${defaults_path}${file_path}" ]; then
        $bb cp -f "${defaults_path}${file_path}" "${HAB_STUDIO_ROOT}${file_path}"
    else
        exit_with "Tried to copy default file for '${file_path}', but could not find one! Please report this error." "${ERR_RUNTIME_CODING_ERROR}"
    fi
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

# The root path of the Habitat file system. If the `$HAB_ROOT_PATH` environment
# variable is set, this value is overridden, otherwise it is set to its default
: "${HAB_ROOT_PATH:=/hab}"
# The root path containing all locally installed packages. This is used in some
# of the hab-studio-type-*.sh scripts
# shellcheck disable=2034
HAB_PKG_PATH=$HAB_ROOT_PATH/pkgs
# The default download root path for package artifacts, used on package
# installation
HAB_CACHE_ARTIFACT_PATH=$HAB_ROOT_PATH/cache/artifacts

# The exit code for a coding error that manifests at runtime
ERR_RUNTIME_CODING_ERROR=70

# The exit code if unmounting a filesystem fails
ERR_UMOUNT_FAILED=80
# The exit code if after a successful unmount, the filesystem is still mounted
ERR_MOUNT_PERSISTS=81
# The exit code if remaining mounted filesystem are found before final studio
# cleanup
ERR_REMAINING_MOUNTS=82

#
bb="$libexec_path/busybox"
#
hab="$libexec_path/hab"
# The current version of Habitat Studio
: "${version:=@version@}"
# The author of this program
author='@author@'
# The short version of the program name which is used in logging output
program=$($bb basename "$0")

ensure_root


# ## CLI Argument Parsing

# Parse command line flags and options.
while getopts ":nNa:k:r:s:t:D:vqVh" opt; do
  case $opt in
    a)
      ARTIFACT_PATH=$OPTARG
      ;;
    n)
      NO_SRC_PATH=true
      ;;
    N)
      NO_ARTIFACT_PATH=true
      ;;
    k)
      HAB_ORIGIN_KEYS=$OPTARG
      ;;
    r)
      HAB_STUDIO_ROOT=$OPTARG
      ;;
    s)
      SRC_PATH=$OPTARG
      ;;
    t)
      STUDIO_TYPE=$OPTARG
      ;;
    D)
      DOCKER=true
      ;;
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

# The source path to be mounted into the Studio, which defaults to current
# working directory
: "${SRC_PATH:=$($bb pwd)}"
# The artifacts cache path to be mounted into the Studio, which defaults to the
# artifact cache path.
: "${ARTIFACT_PATH:=$HAB_CACHE_ARTIFACT_PATH}"
# The directory name of the Studio (which will live under `$HAB_STUDIOS_HOME`).
# It is a directory path turned into a single directory name that can be
# deterministically re-constructed on next program invocation.
dir_name="$(echo "$SRC_PATH" | $bb sed -e 's,^/$,root,' -e 's,^/,,' -e 's,/,--,g' -e 's, ,-,g')"
# The base path under which all Studios are created, which defaults to
# `/hab/studios`.
: "${HAB_STUDIOS_HOME:=/hab/studios}"
# The root path of the Studio, which defaults to
# `$HAB_STUDIOS_HOME/<SRC_PATH_AS_STRING>`.
: "${HAB_STUDIO_ROOT:=$HAB_STUDIOS_HOME/$dir_name}"
# A collection of comma-separated keys to be copied into the Studio's key
# cache directory. If this environment variable is not set, use the value
# from `$HAB_ORIGIN` if set, otherwise, it's empty.
: "${HAB_ORIGIN_KEYS:=${HAB_ORIGIN:-}}"
# The Studio configuration file which is used to determine commands to run,
# extra environment variables, etc. Note that a valid Studio will have this
# file at the root of its filesystem.
studio_config="$HAB_STUDIO_ROOT/.studio"
# The type (flavor, variant, etc.) of Studio. Such types include `default`,
# `stage1`, and `busybox` among others.
: "${STUDIO_TYPE:=}"
# Whether or not to mount the `$ARTIFACT_PATH` into the Studio. An unset or
# empty value mean it is set to false (and therefore will mount
# `$ARTIFACT_PATH`) and any other value is considered set to true (and
# therefore will not mount `$ARTIFACT_PATH`). The choice of this variable name
# is intended to show that it is not default behavior to skip the artifacts
# cache path mounting and the user must explicitly opt-out.
: ${NO_ARTIFACT_PATH:=}
# Whether or not to mount the `$SRC_PATH` into the Studio. An unset or empty
# value mean it is set to false (and therefore will mount `$SRC_PATH`) and any
# other value is considered set to true (and therefore will not mount
# `$SRC_PATH`). The choice of this variable name is intended to show that it is
# not default behavior to skip the source path mounting and the user must
# explicitly opt-out.
: ${NO_SRC_PATH:=}
# Whether to use a docker container as the environment to build the studio in
: ${DOCKER:=}
# Whether or not to mount filesystem in the Studio. An unset or empty value
# means it is set to false (and therefore will mount filesystems) and any other
# value is considered set to true (and therefore will not mount filesystems).
: "${NO_MOUNT:=}"
# Whether or not more verbose output has been requested. An unset or empty
# value means it is set to false and any other value is considered set or true.
: ${VERBOSE:=}
set_v_flag
# Whether or not less output has been requested. An unset or empty value means
# it is set to false and any other value is considered set or true.
: ${QUIET:=}

export VERBOSE QUIET

# Next, determine the subcommand and delegate its behavior to the appropriate
# function. Note that the multiple word fragments for each case result in a
# "fuzzy matching" behavior, meaning that `studio e` is equivalent to `studio
# enter`.
case ${1:-} in
  n|ne|new)
    shift
    subcommand_new "$@"
    ;;
  rm)
    shift
    subcommand_rm "$@"
    ;;
  e|en|ent|ente|enter)
    shift
    subcommand_enter "$@"
    ;;
  b|bu|bui|buil|build)
    shift
    subcommand_build "$@"
    ;;
  r|ru|run)
    shift
    subcommand_run "$@"
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
