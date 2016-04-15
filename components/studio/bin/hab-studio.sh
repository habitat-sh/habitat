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
  export DEBUG
fi


# ## Help/Usage functions

# **Internal** Prints help and usage information. Straight forward, no?
print_help() {
  printf -- "$program $version

$author

Habitat Studios - Plan for success!

USAGE:
        $program [FLAGS] [OPTIONS] <SUBCOMMAND> [ARG ..]

FLAGS:
    -h  Prints this message
    -n  Do not mount the source path into the Studio (default: mount the path)
    -q  Prints less output for better use in scripts
    -v  Prints more verbose output
    -V  Prints version information

OPTIONS:
    -r <STUDIO_ROOT>  Sets a Studio root (default: /opt/studio)
    -s <SRC_PATH>     Sets the source path (default: \$PWD)
    -t <STUDIO_TYPE>  Sets a Studio type when creating (default: default)
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
    NO_SRC_PATH   If set, do not mount source path (\`-n' flag takes precedence)
    QUIET         Prints less output (\`-q' flag takes precedence)
    SRC_PATH      Sets the source path (\`-s' option takes precedence)
    STUDIO_ROOT   Sets a Studio root (\`-r' option takes precedence)
    STUDIO_TYPE   Sets a Studio type when creating (\`-t' option takes precedence)
    STUDIOS_HOME  Sets a home path for all Studios (default: /opt/studios)
    VERBOSE       Prints more verbose output (\`-v' flag takes precedence)

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


# ## Subcommand functions
#
# These are the implmentations for each subcommand in the program.

# **Internal** Creates a new Studio.
new_studio() {
  # Check if a pre-existing Studio configuration is found and use that to
  # determine the type
  if [ -s $studio_config ]; then
    . $studio_config
    STUDIO_TYPE=$studio_type
  fi

  # Validate the type specified is valid and set a default if unset
  case "${STUDIO_TYPE:-unset}" in
    unset|default)
      # Set the default/unset type
      STUDIO_TYPE=default
      ;;
    busybox|stage1|baseimage)
      # Confirmed valid types
      ;;
    *)
      # Everything else is invalid
      exit_with "Invalid Studio type: $STUDIO_TYPE" 2
      ;;
  esac

  info "Creating Studio at $STUDIO_ROOT ($STUDIO_TYPE)"

  # Set the verbose flag (i.e. `-v`) for any coreutils-like commands if verbose
  # mode was requested
  if [ -n "$VERBOSE" ]; then
    local v="-v"
  else
    local v=
  fi

  # Mount filesystems

  $bb mkdir -p $v $STUDIO_ROOT/dev
  $bb mkdir -p $v $STUDIO_ROOT/proc
  $bb mkdir -p $v $STUDIO_ROOT/sys
  $bb mkdir -p $v $STUDIO_ROOT/run
  $bb mkdir -p $v $STUDIO_ROOT/var/run

  # Make  a `/dev/console` device, if it doesn't exist
  if [ ! -r $STUDIO_ROOT/dev/console ]; then
    $bb mknod -m 600 $STUDIO_ROOT/dev/console c 5 1
  fi
  # Make  a `/dev/null` device, if it doesn't exist
  if [ ! -r $STUDIO_ROOT/dev/null ]; then
    $bb mknod -m 666 $STUDIO_ROOT/dev/null c 1 3
  fi

  # Unless `$NO_MOUNT` is set, mount filesystems such as `/dev`, `/proc`, and
  # company. If the mount already exists, skip it to be all idempotent and
  # nerdy like that
  if [ -z "${NO_MOUNT}" ]; then
    if ! $bb mount | $bb grep -q "on $STUDIO_ROOT/dev type"; then
      $bb mount $v --bind /dev $STUDIO_ROOT/dev
    fi

    if ! $bb mount | $bb grep -q "on $STUDIO_ROOT/dev/pts type"; then
      $bb mount $v -t devpts devpts $STUDIO_ROOT/dev/pts -o gid=5,mode=620
    fi
    if ! $bb mount | $bb grep -q "on $STUDIO_ROOT/proc type"; then
      $bb mount $v -t proc proc $STUDIO_ROOT/proc
    fi
    if ! $bb mount | $bb grep -q "on $STUDIO_ROOT/sys type"; then
      $bb mount $v -t sysfs sysfs $STUDIO_ROOT/sys
    fi
    if ! $bb mount | $bb grep -q "on $STUDIO_ROOT/run type"; then
      $bb mount $v -t tmpfs tmpfs $STUDIO_ROOT/run
    fi
    if [ -e /var/run/docker.sock ]; then
      if ! $bb mount | $bb grep -q "on $STUDIO_ROOT/var/run/docker.sock type"; then
        $bb touch $STUDIO_ROOT/var/run/docker.sock
        $bb mount $v --bind /var/run/docker.sock $STUDIO_ROOT/var/run/docker.sock
      fi
    fi

    if [ -h "$STUDIO_ROOT/dev/shm" ]; then
      $bb mkdir -p $v $STUDIO_ROOT/$($bb readlink $STUDIO_ROOT/dev/shm)
    fi
  fi

  # Create root filesystem

  $bb mkdir -p $v $STUDIO_ROOT/bin
  $bb mkdir -p $v $STUDIO_ROOT/etc
  $bb mkdir -p $v $STUDIO_ROOT/home
  $bb mkdir -p $v $STUDIO_ROOT/lib
  $bb mkdir -p $v $STUDIO_ROOT/mnt
  $bb mkdir -p $v $STUDIO_ROOT/opt
  $bb mkdir -p $v $STUDIO_ROOT/sbin

  $bb install -d $v -m 0750 $STUDIO_ROOT/root
  $bb install -d $v -m 1777 $STUDIO_ROOT/tmp $STUDIO_ROOT/var/tmp

  $bb mkdir -p $v $STUDIO_ROOT/usr/bin
  $bb mkdir -p $v $STUDIO_ROOT/usr/include
  $bb mkdir -p $v $STUDIO_ROOT/usr/lib
  $bb mkdir -p $v $STUDIO_ROOT/usr/libexec
  $bb mkdir -p $v $STUDIO_ROOT/usr/sbin

  $bb mkdir -p $v $STUDIO_ROOT/usr/share/doc
  $bb mkdir -p $v $STUDIO_ROOT/usr/share/info
  $bb mkdir -p $v $STUDIO_ROOT/usr/share/locale
  $bb mkdir -p $v $STUDIO_ROOT/usr/share/man
  $bb mkdir -p $v $STUDIO_ROOT/usr/share/man/man1
  $bb mkdir -p $v $STUDIO_ROOT/usr/share/man/man2
  $bb mkdir -p $v $STUDIO_ROOT/usr/share/man/man3
  $bb mkdir -p $v $STUDIO_ROOT/usr/share/man/man4
  $bb mkdir -p $v $STUDIO_ROOT/usr/share/man/man5
  $bb mkdir -p $v $STUDIO_ROOT/usr/share/man/man6
  $bb mkdir -p $v $STUDIO_ROOT/usr/share/man/man7
  $bb mkdir -p $v $STUDIO_ROOT/usr/share/man/man8
  $bb mkdir -p $v $STUDIO_ROOT/usr/share/misc
  $bb mkdir -p $v $STUDIO_ROOT/usr/share/terminfo
  $bb mkdir -p $v $STUDIO_ROOT/usr/share/zoneinfo

  # If the system is 64-bit, a few symlinks will be required
  case $($bb uname -m) in
  x86_64)
    $bb ln -sf $v lib $STUDIO_ROOT/lib64
    $bb ln -sf $v lib $STUDIO_ROOT/usr/lib64
    ;;
  esac

  $bb mkdir -p $v $STUDIO_ROOT/var
  $bb mkdir -p $v $STUDIO_ROOT/var/log
  $bb mkdir -p $v $STUDIO_ROOT/var/mail
  $bb mkdir -p $v $STUDIO_ROOT/var/spool

  #$bb ln -sf $v /run $STUDIO_ROOT/var/run
  $bb ln -sf $v /run/lock $STUDIO_ROOT/var/lock

  $bb mkdir -p $v $STUDIO_ROOT/var/opt
  $bb mkdir -p $v $STUDIO_ROOT/var/cache
  $bb mkdir -p $v $STUDIO_ROOT/var/lib/color
  $bb mkdir -p $v $STUDIO_ROOT/var/lib/misc
  $bb mkdir -p $v $STUDIO_ROOT/var/lib/locate
  $bb mkdir -p $v $STUDIO_ROOT/var/local

  $bb ln -sf $v /proc/self/mounts $STUDIO_ROOT/etc/mtab

  $bb touch $STUDIO_ROOT/var/log/btmp
  $bb touch $STUDIO_ROOT/var/log/lastlog
  $bb touch $STUDIO_ROOT/var/log/wtmp
  $bb chgrp $v 13 $STUDIO_ROOT/var/log/lastlog
  $bb chmod $v 664 $STUDIO_ROOT/var/log/lastlog
  $bb chmod $v 600 $STUDIO_ROOT/var/log/btmp

  # Load the appropriate type strategy to complete the setup
  . $libexec_path/hab-studio-type-${STUDIO_TYPE}.sh

  # If `/etc/passwd` is not present, create a minimal version to satisfy
  # some software when being built
  if [ ! -f "$STUDIO_ROOT/etc/passwd" ]; then
    if [ -n "$VERBOSE" ]; then
      echo "> Creating minimal /etc/passwd"
    fi
    $bb cat > $STUDIO_ROOT/etc/passwd << "EOF"
root:x:0:0:root:/root:/bin/sh
bin:x:1:1:bin:/dev/null:/bin/false
daemon:x:6:6:Daemon User:/dev/null:/bin/false
messagebus:x:18:18:D-Bus Message Daemon User:/var/run/dbus:/bin/false
nobody:x:99:99:Unprivileged User:/dev/null:/bin/false
EOF
  fi

  # If `/etc/group` is not present, create a minimal version to satisfy
  # some software when being built
  if [ ! -f "$STUDIO_ROOT/etc/group" ]; then
    if [ -n "$VERBOSE" ]; then
      echo "> Creating minimal /etc/group"
    fi
    $bb cat > $STUDIO_ROOT/etc/group << "EOF"
root:x:0:
bin:x:1:daemon
sys:x:2:
kmem:x:3:
tape:x:4:
tty:x:5:
daemon:x:6:
floppy:x:7:
disk:x:8:
lp:x:9:
dialout:x:10:
audio:x:11:
video:x:12:
utmp:x:13:
usb:x:14:
cdrom:x:15:
adm:x:16:
messagebus:x:18:
systemd-journal:x:23:
input:x:24:
mail:x:34:
nogroup:x:99:
users:x:999:
EOF
  fi

  # Copy minimal networking and DNS resolution configuration files into the
  # Studio filesystem so that commands such as `wget(1)` will work
  for f in /etc/hosts /etc/resolv.conf; do
    $bb mkdir -p $v $($bb dirname $f)
    $bb cp $v $f $STUDIO_ROOT$f
  done

  # Invoke the type's implementation
  finish_setup

  # This prompt tells us what kind of studio we're in!
  prompt='[\#]['$studio_type':\w:\$?]\$\040 '
  studio_enter_environment="$studio_enter_environment PS1=$prompt"

  # Add a Studio configuration file at the root of the filesystem
  $bb cat <<EOF > $studio_config
studio_type="$studio_type"
studio_path="$studio_path"
studio_env_command="$studio_env_command"
studio_enter_environment="$studio_enter_environment"
studio_enter_command="$studio_enter_command"
studio_build_environment="$studio_build_environment"
studio_build_command="$studio_build_command"
studio_run_environment="$studio_run_environment"
EOF

  # If `/etc/profile` is not present, create a minimal version with convenient
  # helper functions
  local pfile="$STUDIO_ROOT/etc/profile"
  if [ ! -f "$pfile" ] || ! $bb grep -q '^record() {$' "$pfile"; then
    if [ -n "$VERBOSE" ]; then
      echo "> Creating /etc/profile"
    fi
    $bb cat >> "$pfile" <<'PROFILE'
# Setting the user file-creation mask (umask) to 022 ensures that newly created
# files and directories are only writable by their owner, but are readable and
# executable by anyone (assuming default modes are used by the open(2) system
# call, new files will end up with permission mode 644 and directories with
# mode 755).
umask 022

# Colorize ls by default
if command -v dircolors > /dev/null; then
  eval "$(dircolors -b)"
fi
alias ls="ls --color=auto"

record() {
  (if [ -n "${DEBUG:-}" ]; then set -x; fi; unset DEBUG
    if [ -z "${1:-}" ]; then
      >&2 echo "Usage: record <SESSION> [CMD [ARG ..]]"
      return 1
    fi
    name=$1; shift
    cmd="${1:-${SHELL:-sh} -l}"; shift
    bb=${BUSYBOX:-}
    env="$($bb env \
      | $bb sed -e "s,^,'," -e "s,$,'," -e 's,0;32m,0;31m,g' \
      | $bb tr '\n' ' ')"
    log="${LOGDIR:-/src/log}/${name}.$($bb date -u +%Y-%m-%d-%H%M%S).log"
    $bb mkdir -p $($bb dirname $log)
    unset BUSYBOX LOGDIR

    $bb script -c "$bb env -i $env $cmd $*" $log
  ); return $?
}

cd /src

PROFILE
  fi

  $bb mkdir -p $v $STUDIO_ROOT/src
  # Mount the `$SRC_PATH` under `/src` in the Studio, unless either `$NO_MOUNT`
  # or `$NO_SRC_PATH` are set
  if [ -z "${NO_MOUNT}" -a -z "${NO_SRC_PATH}" ]; then
    if ! $bb mount | $bb grep -q "on $STUDIO_ROOT/src type"; then
      $bb mount $v --bind $SRC_PATH $STUDIO_ROOT/src
    fi
  fi
}

# **Internal** Interactively enter a Studio.
enter_studio() {
  # If a non-zero sized Studio configuration is not found, exit the program.
  if [ ! -s $STUDIO_ROOT/.studio ]; then
    exit_with "Directory $STUDIO_ROOT does not appear to be a Studio, aborting" 5
  fi
  # Check if a pre-existing Studio configuration is found and use that to
  # determine the type. If no config is found, set the type to `unknown`.
  if [ -s $studio_config ]; then
    . $studio_config
    STUDIO_TYPE=$studio_type
  else
    STUDIO_TYPE=unknown
  fi

  local env="$(chroot_env "$studio_path" "$studio_enter_environment")"

  info "Entering Studio at $STUDIO_ROOT ($STUDIO_TYPE)"
  echo

  if [ -n "$VERBOSE" ]; then
    set -x
  fi

  # Become the `chroot` process
  exec $bb chroot "$STUDIO_ROOT" \
    $studio_env_command -i $env $studio_enter_command $*
}

# **Internal** Run a build command using a Studio.
build_studio() {
  # If a non-zero sized Studio configuration is not found, exit the program.
  if [ ! -s $STUDIO_ROOT/.studio ]; then
    exit_with "Directory $STUDIO_ROOT does not appear to be a Studio, aborting" 5
  fi
  # Check if a pre-existing Studio configuration is found and use that to
  # determine the type. If no config is found, set the type to `unknown`.
  if [ -s $studio_config ]; then
    . $studio_config
    STUDIO_TYPE=$studio_type
  else
    STUDIO_TYPE=unknown
  fi

  # If a build command is not set, then this type does not support the `build`
  # subcommand and should abort.
  if [ -z "$studio_build_command" ]; then
    exit_with "Studio at $STUDIO_ROOT ($STUDIO_TYPE) does not support 'build'" 10
  fi

  local env="$(chroot_env "$studio_path" "$studio_build_environment")"

  info "Building '$*' in Studio at $STUDIO_ROOT ($STUDIO_TYPE)"

  if [ -n "$VERBOSE" ]; then
    set -x
  fi

  # Run the build command in the `chroot` environment
  echo $studio_build_command $* | $bb chroot "$STUDIO_ROOT" \
    $studio_env_command -i $env $studio_run_command
}

# **Internal** Run an arbitrary command in a Studio.
run_studio() {
  # If a non-zero sized Studio configuration is not found, exit the program.
  if [ ! -s $STUDIO_ROOT/.studio ]; then
    exit_with "Directory $STUDIO_ROOT does not appear to be a Studio, aborting" 5
  fi
  # Check if a pre-existing Studio configuration is found and use that to
  # determine the type. If no config is found, set the type to `unknown`.
  if [ -s $studio_config ]; then
    . $studio_config
    STUDIO_TYPE=$studio_type
  else
    STUDIO_TYPE=unknown
  fi

  local env="$(chroot_env "$studio_path" "$studio_run_environment")"

  info "Running '$*' in Studio at $STUDIO_ROOT ($STUDIO_TYPE)"

  if [ -n "$VERBOSE" ]; then
    set -x
  fi

  # Run the command in the `chroot` environment
  echo $* | $bb chroot "$STUDIO_ROOT" \
    $studio_env_command -i $env $studio_run_command
}

# **Internal** Destroy a Studio.
rm_studio() {
  # Check if a pre-existing Studio configuration is found and use that to
  # determine the type. If no config is found, set the type to `unknown`.
  if [ -s $studio_config ]; then
    . $studio_config
    STUDIO_TYPE=$studio_type
  else
    STUDIO_TYPE=unknown
  fi

  info "Destroying Studio at $STUDIO_ROOT ($STUDIO_TYPE)"

  # Set the verbose flag (i.e. `-v`) for any coreutils-like commands if verbose
  # mode was requested
  if [ -n "$VERBOSE" ]; then
    local v="-v"
  else
    local v=
  fi

  # Unmount filesystems that were previously set up in, but only if they are
  # currently mounted. You know, so you can run this all day long, like, for
  # fun and stuff.

  if $bb mount | $bb grep -q "on $STUDIO_ROOT/src type"; then
    $bb umount $v -l $STUDIO_ROOT/src
  fi

  if $bb mount | $bb grep -q "on $STUDIO_ROOT/run type"; then
    $bb umount $v $STUDIO_ROOT/run
  fi

  if $bb mount | $bb grep -q "on $STUDIO_ROOT/sys type"; then
    $bb umount $v $STUDIO_ROOT/sys
  fi

  if $bb mount | $bb grep -q "on $STUDIO_ROOT/proc type"; then
    $bb umount $v $STUDIO_ROOT/proc
  fi

  if $bb mount | $bb grep -q "on $STUDIO_ROOT/dev/pts type"; then
    $bb umount $v $STUDIO_ROOT/dev/pts
  fi

  if $bb mount | $bb grep -q "on $STUDIO_ROOT/dev type"; then
    $bb umount $v -l $STUDIO_ROOT/dev
  fi

  if $bb mount | $bb grep -q "on $STUDIO_ROOT/var/run/docker.sock type"; then
    $bb umount $v -l $STUDIO_ROOT/var/run/docker.sock
  fi

  # If a Studio root directory exists, but does not contain a Studio
  # configuration, we're going to abort rather than let the program attempt to
  # recursively delete the directory tree. It's a super small detail, but
  # there's an attempt at safety.
  if [ -d $STUDIO_ROOT -a ! -f $studio_config ]; then
    exit_with "Directory $STUDIO_ROOT does not appear to be a Studio, aborting" 5
  fi

  # Remove remaining filesystem
  $bb rm -rf $v $STUDIO_ROOT
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

# **Internal** Builds up the environment set to pass to an `env(1)` command for
# use in a `chroot` environment which is printed on stdout.
chroot_env() {
  local studio_path="$1"
  local extra_env="$2"

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
  # If a Habitat Depot URL is set, then propagate it into the Studio's
  # environment.
  if [ -n "${BLDR_REPO:-}" ]; then
    env="$env BLDR_REPO=$BLDR_REPO"
  fi
  # If HTTP proxy variables are detected in the current environment, propagate
  # them into the Studio's environment.
  if [ -n "${http_proxy:-}" ]; then
    env="$env http_proxy=$http_proxy"
  fi
  if [ -n "${https_proxy:-}" ]; then
    env="$env https_proxy=$https_proxy"
  fi

  echo "$env"
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

# The root path of the Habitat file system. If the `$HAB_ROOT_PATH` environment
# variable is set, this value is overridden, otherwise it is set to its default
: ${HAB_ROOT_PATH:=/hab}
# The root path containing all locally installed packages
HAB_PKG_PATH=$HAB_ROOT_PATH/pkgs
# The default download root path for package artifacts, used on package
# installation
HAB_CACHE_ARTIFACT_PATH=$HAB_ROOT_PATH/cache/artifacts

#
bb="$libexec_path/busybox"
#
bpm="$libexec_path/hab-bpm"
# The current version of Habitat Studio
version='@version@'
# The author of this program
author='@author@'
# The short version of the program name which is used in logging output
program=$($bb basename $0)


# ## CLI Argument Parsing

# Parse command line flags and options.
while getopts ":nr:s:t:vqVh" opt; do
  case $opt in
    n)
      NO_SRC_PATH=true
      ;;
    r)
      STUDIO_ROOT=$OPTARG
      ;;
    s)
      SRC_PATH=$OPTARG
      ;;
    t)
      STUDIO_TYPE=$OPTARG
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
: ${SRC_PATH:=$($bb pwd)}
# The directory name of the Studio (which will live under `$STUDIOS_HOME`). It
# is a directoy path turned into a single directory name that can be
# deterministically re-constructed on next program invocation.
dir_name="$(echo $SRC_PATH | $bb sed -e 's,^/$,root,' -e 's,^/,,' -e 's,/,--,g')"
# The base path udner which all Studios are created, which defaults to
# `/opt/studios`.
: ${STUDIOS_HOME:=/opt/studios}
# The root path of the Studio, which defaults to
# `$STUDIOS_HOME/<SRC_PATH_AS_STRING>`.
: ${STUDIO_ROOT:=$STUDIOS_HOME/$dir_name}
# The Studio configuration file which is used to determine commands to run,
# extra environment variables, etc. Note that a valid Studio will have this
# file at the root of its filesystem.
studio_config="$STUDIO_ROOT/.studio"
# The type (flavor, variant, etc.) of Studio. Such types include `default`,
# `stage1`, and `busybox` among others.
: ${STUDIO_TYPE:=}
# Whether or not to mount the `$SRC_PATH` into the Studio. An unset or empty
# value mean it is set to false (and therefore will mount `$SRC_PATH`) and any
# other value is considered set to true (and therefore will not mount
# `$SRC_PATH`). The choice of this variable name is intended to show that it is
# not default behavior to skip the source path mounting and the user must
# explicitly opt-out.
: ${NO_SRC_PATH:=}
# Whether or not to mount filesystem in the Studio. An unset or empty value
# means it is set to false (and therefore will mount filesystems) and any other
# value is considered set to true (and therefore will not mount filesystems).
: ${NO_MOUNT:=}
# Whether or not more verbose output has been requested. An unset or empty
# value means it is set to false and any other value is considered set or true.
: ${VERBOSE:=}
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
    new_studio
    ;;
  rm)
    shift
    rm_studio $*
    ;;
  e|en|ent|ente|enter)
    shift
    new_studio
    enter_studio $*
    ;;
  b|bu|bui|buil|build)
    shift
    new_studio
    build_studio $*
    ;;
  r|ru|run)
    shift
    new_studio
    run_studio $*
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
