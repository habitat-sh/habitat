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

COMMON FLAGS:
    -h  Prints this message
    -n  Do not mount the source path into the Studio (default: mount the path)
    -N  Do not mount the source artifact cache path into the Studio (default: mount the path)
    -q  Prints less output for better use in scripts
    -v  Prints more verbose output
    -V  Prints version information
    -w  Use a Windows studio instead of a docker studio (only available on windows)

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
    ARTIFACT_PATH       Sets the source artifact cache path (\`-a' option overrides)
    HAB_NOCOLORING      Disables text coloring mode despite TERM capabilities
    HAB_NONINTERACTIVE  Disables interactive progress bars despite tty
    HAB_ORIGIN          Propagates this variable into any studios
    HAB_ORIGIN_KEYS     Installs secret keys (\`-k' option overrides)
    HAB_STUDIOS_HOME    Sets a home path for all Studios (default: /hab/studios)
    HAB_STUDIO_ROOT     Sets a Studio root (\`-r' option overrides)
    NO_ARTIFACT_PATH    If set, do not mount the source artifact cache path (\`-N' flag overrides)
    NO_SRC_PATH         If set, do not mount the source path (\`-n' flag overrides)
    QUIET               Prints less output (\`-q' flag overrides)
    SRC_PATH            Sets the source path (\`-s' option overrides)
    STUDIO_TYPE         Sets a Studio type when creating (\`-t' option overrides)
    VERBOSE             Prints more verbose output (\`-v' flag overrides)
    http_proxy          Sets an http_proxy environment variable inside the Studio
    https_proxy         Sets an https_proxy environment variable inside the Studio
    no_proxy            Sets a no_proxy environment variable inside the Studio

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
  printf -- "${program}-build $version

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
  printf -- "${program}-enter $version

$author

Habitat Studios - interactively enter a Studio

USAGE:
        $program [COMMON_FLAGS] [COMMON_OPTIONS] enter

"
}

print_new_help() {
  printf -- "${program}-new $version

$author

Habitat Studios - create a new Studio

USAGE:
        $program [COMMON_FLAGS] [COMMON_OPTIONS] new

"
}

print_rm_help() {
  printf -- "${program}-rm $version

$author

Habitat Studios - destroy a Studio

USAGE:
        $program [COMMON_FLAGS] [COMMON_OPTIONS] rm

"
}

print_run_help() {
  printf -- "${program}-run $version

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
  local opt

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

  new_studio
}

# **Internal** Parses options and flags for `rm` subcommand.
subcommand_rm() {
  local opt

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

  rm_studio $*
}

# **Internal** Parses options and flags for `enter` subcommand.
subcommand_enter() {
  local opt

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

  new_studio
  enter_studio $*
}

# **Internal** Parses options and flags for `build` subcommand.
subcommand_build() {
  local opt
  local reuse

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

  if [ -z "${reuse:-}" ]; then
    _STUDIO_TYPE="$STUDIO_TYPE"
    rm_studio
    STUDIO_TYPE="$_STUDIO_TYPE"
    unset _STUDIO_TYPE
  fi
  new_studio
  build_studio $*
}

# **Internal** Parses options and flags for `run` subcommand.
subcommand_run() {
  local opt

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

  new_studio
  run_studio $*
}

# **Internal** Creates a new Studio.
new_studio() {
  # Check if a pre-existing Studio configuration is found and use that to
  # determine the type
  if [ -s "$studio_config" ]; then
    . "$studio_config"
    STUDIO_TYPE=$studio_type
  fi

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
  $bb mkdir -p $HAB_STUDIO_ROOT
  HAB_STUDIO_ROOT="$($bb readlink -f $HAB_STUDIO_ROOT)"

  info "Creating Studio at $HAB_STUDIO_ROOT ($STUDIO_TYPE)"

  # Set the verbose flag (i.e. `-v`) for any coreutils-like commands if verbose
  # mode was requested
  if [ -n "$VERBOSE" ]; then
    local v="-v"
  else
    local v=
  fi

  # Mount filesystems

  $bb mkdir -p $v $HAB_STUDIO_ROOT/dev
  $bb mkdir -p $v $HAB_STUDIO_ROOT/proc
  $bb mkdir -p $v $HAB_STUDIO_ROOT/sys
  $bb mkdir -p $v $HAB_STUDIO_ROOT/run
  $bb mkdir -p $v $HAB_STUDIO_ROOT/var/run

  # Make  a `/dev/console` device, if it doesn't exist
  if [ ! -r $HAB_STUDIO_ROOT/dev/console ]; then
    $bb mknod -m 600 $HAB_STUDIO_ROOT/dev/console c 5 1
  fi
  # Make  a `/dev/null` device, if it doesn't exist
  if [ ! -r $HAB_STUDIO_ROOT/dev/null ]; then
    $bb mknod -m 666 $HAB_STUDIO_ROOT/dev/null c 1 3
  fi

  # Unless `$NO_MOUNT` is set, mount filesystems such as `/dev`, `/proc`, and
  # company. If the mount already exists, skip it to be all idempotent and
  # nerdy like that
  if [ -z "${NO_MOUNT}" ]; then
    if ! $bb mount | $bb grep -q "on $HAB_STUDIO_ROOT/dev type"; then
      $bb mount $v --bind /dev $HAB_STUDIO_ROOT/dev
    fi

    if ! $bb mount | $bb grep -q "on $HAB_STUDIO_ROOT/dev/pts type"; then
      $bb mount $v -t devpts devpts $HAB_STUDIO_ROOT/dev/pts -o gid=5,mode=620
    fi
    if ! $bb mount | $bb grep -q "on $HAB_STUDIO_ROOT/proc type"; then
      $bb mount $v -t proc proc $HAB_STUDIO_ROOT/proc
    fi
    if ! $bb mount | $bb grep -q "on $HAB_STUDIO_ROOT/sys type"; then
      $bb mount $v -t sysfs sysfs $HAB_STUDIO_ROOT/sys
    fi
    if ! $bb mount | $bb grep -q "on $HAB_STUDIO_ROOT/run type"; then
      $bb mount $v -t tmpfs tmpfs $HAB_STUDIO_ROOT/run
    fi
    if [ -e /var/run/docker.sock ]; then
      if ! $bb mount | $bb grep -q "on $HAB_STUDIO_ROOT/var/run/docker.sock type"; then
        $bb touch $HAB_STUDIO_ROOT/var/run/docker.sock
        $bb mount $v --bind /var/run/docker.sock $HAB_STUDIO_ROOT/var/run/docker.sock
      fi
    fi

    if [ -h "$HAB_STUDIO_ROOT/dev/shm" ]; then
      $bb mkdir -p $v $HAB_STUDIO_ROOT/$($bb readlink $HAB_STUDIO_ROOT/dev/shm)
    fi

    # Mount the `$ARTIFACT_PATH` under `/hab/cache/artifacts` in the Studio,
    # unless `$NO_ARTIFACT_PATH` are set
    if [ -z "${NO_ARTIFACT_PATH}" ]; then
      local studio_artifact_path
      studio_artifact_path="${HAB_STUDIO_ROOT}${HAB_CACHE_ARTIFACT_PATH}"
      if ! $bb mount | $bb grep -q "on $studio_artifact_path type"; then
        $bb mkdir -p $v $studio_artifact_path
        $bb mount $v --bind $ARTIFACT_PATH $studio_artifact_path
      fi
    fi
  fi

  # Create root filesystem

  $bb mkdir -p $v $HAB_STUDIO_ROOT/bin
  $bb mkdir -p $v $HAB_STUDIO_ROOT/etc
  $bb mkdir -p $v $HAB_STUDIO_ROOT/home
  $bb mkdir -p $v $HAB_STUDIO_ROOT/lib
  $bb mkdir -p $v $HAB_STUDIO_ROOT/mnt
  $bb mkdir -p $v $HAB_STUDIO_ROOT/opt
  $bb mkdir -p $v $HAB_STUDIO_ROOT/sbin

  $bb install -d $v -m 0750 $HAB_STUDIO_ROOT/root
  $bb install -d $v -m 1777 $HAB_STUDIO_ROOT/tmp $HAB_STUDIO_ROOT/var/tmp

  $bb mkdir -p $v $HAB_STUDIO_ROOT/usr/bin
  $bb mkdir -p $v $HAB_STUDIO_ROOT/usr/include
  $bb mkdir -p $v $HAB_STUDIO_ROOT/usr/lib
  $bb mkdir -p $v $HAB_STUDIO_ROOT/usr/libexec
  $bb mkdir -p $v $HAB_STUDIO_ROOT/usr/sbin

  $bb mkdir -p $v $HAB_STUDIO_ROOT/usr/share/doc
  $bb mkdir -p $v $HAB_STUDIO_ROOT/usr/share/info
  $bb mkdir -p $v $HAB_STUDIO_ROOT/usr/share/locale
  $bb mkdir -p $v $HAB_STUDIO_ROOT/usr/share/man
  $bb mkdir -p $v $HAB_STUDIO_ROOT/usr/share/man/man1
  $bb mkdir -p $v $HAB_STUDIO_ROOT/usr/share/man/man2
  $bb mkdir -p $v $HAB_STUDIO_ROOT/usr/share/man/man3
  $bb mkdir -p $v $HAB_STUDIO_ROOT/usr/share/man/man4
  $bb mkdir -p $v $HAB_STUDIO_ROOT/usr/share/man/man5
  $bb mkdir -p $v $HAB_STUDIO_ROOT/usr/share/man/man6
  $bb mkdir -p $v $HAB_STUDIO_ROOT/usr/share/man/man7
  $bb mkdir -p $v $HAB_STUDIO_ROOT/usr/share/man/man8
  $bb mkdir -p $v $HAB_STUDIO_ROOT/usr/share/misc
  $bb mkdir -p $v $HAB_STUDIO_ROOT/usr/share/terminfo
  $bb mkdir -p $v $HAB_STUDIO_ROOT/usr/share/zoneinfo

  # If the system is 64-bit, a few symlinks will be required
  case $($bb uname -m) in
  x86_64)
    $bb ln -sf $v lib $HAB_STUDIO_ROOT/lib64
    $bb ln -sf $v lib $HAB_STUDIO_ROOT/usr/lib64
    ;;
  esac

  $bb mkdir -p $v $HAB_STUDIO_ROOT/var
  $bb mkdir -p $v $HAB_STUDIO_ROOT/var/log
  $bb mkdir -p $v $HAB_STUDIO_ROOT/var/mail
  $bb mkdir -p $v $HAB_STUDIO_ROOT/var/spool

  #$bb ln -sf $v /run $HAB_STUDIO_ROOT/var/run
  $bb ln -sf $v /run/lock $HAB_STUDIO_ROOT/var/lock

  $bb mkdir -p $v $HAB_STUDIO_ROOT/var/opt
  $bb mkdir -p $v $HAB_STUDIO_ROOT/var/cache
  $bb mkdir -p $v $HAB_STUDIO_ROOT/var/lib/color
  $bb mkdir -p $v $HAB_STUDIO_ROOT/var/lib/misc
  $bb mkdir -p $v $HAB_STUDIO_ROOT/var/lib/locate
  $bb mkdir -p $v $HAB_STUDIO_ROOT/var/local

  $bb ln -sf $v /proc/self/mounts $HAB_STUDIO_ROOT/etc/mtab

  $bb touch $HAB_STUDIO_ROOT/var/log/btmp
  $bb touch $HAB_STUDIO_ROOT/var/log/lastlog
  $bb touch $HAB_STUDIO_ROOT/var/log/wtmp
  $bb chgrp $v 13 $HAB_STUDIO_ROOT/var/log/lastlog
  $bb chmod $v 664 $HAB_STUDIO_ROOT/var/log/lastlog
  $bb chmod $v 600 $HAB_STUDIO_ROOT/var/log/btmp

  # Load the appropriate type strategy to complete the setup
  . $libexec_path/hab-studio-type-${STUDIO_TYPE}.sh

  # If `/etc/passwd` is not present, create a minimal version to satisfy
  # some software when being built
  if [ ! -f "$HAB_STUDIO_ROOT/etc/passwd" ]; then
    if [ -n "$VERBOSE" ]; then
      echo "> Creating minimal /etc/passwd"
    fi
    $bb cat > $HAB_STUDIO_ROOT/etc/passwd << "EOF"
root:x:0:0:root:/root:/bin/sh
bin:x:1:1:bin:/dev/null:/bin/false
daemon:x:6:6:Daemon User:/dev/null:/bin/false
messagebus:x:18:18:D-Bus Message Daemon User:/var/run/dbus:/bin/false
nobody:x:99:99:Unprivileged User:/dev/null:/bin/false
EOF
  fi

  # If `/etc/group` is not present, create a minimal version to satisfy
  # some software when being built
  if [ ! -f "$HAB_STUDIO_ROOT/etc/group" ]; then
    if [ -n "$VERBOSE" ]; then
      echo "> Creating minimal /etc/group"
    fi
    $bb cat > $HAB_STUDIO_ROOT/etc/group << "EOF"
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
    $bb cp $v $f $HAB_STUDIO_ROOT$f
  done

  # Invoke the type's implementation
  finish_setup

  # Add a Studio configuration file at the root of the filesystem
  $bb cat <<EOF > "$studio_config"
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
  # helper functions. "bare" studio doesn't need an /etc/profile
  if [ "$STUDIO_TYPE" != "bare" ]; then
    local pfile="$HAB_STUDIO_ROOT/etc/profile"
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
alias ll="ls -l"
alias la="ls -al"

# Set a prompt which tells us what kind of Studio we're in
if [ "${HAB_NOCOLORING:-}" = "true" ]; then
  PS1='[\#]['${STUDIO_TYPE:-unknown}':\w:`echo -n $?`]\$ '
else
  case "${TERM:-}" in
  *term | xterm-* | rxvt | screen | screen-*)
    PS1='\[\e[0;32m\][\[\e[0;36m\]\#\[\e[0;32m\]]['${STUDIO_TYPE:-unknown}':\[\e[0;35m\]\w\[\e[0;32m\]:\[\e[1;37m\]`echo -n $?`\[\e[0;32m\]]\$\[\e[0m\] '
    ;;
  *)
    PS1='[\#]['${STUDIO_TYPE:-unknown}':\w:`echo -n $?`]\$ '
    ;;
  esac
fi

record() {
  (if [ -n "${DEBUG:-}" ]; then set -x; fi; unset DEBUG
    if [ -z "${1:-}" ]; then
      >&2 echo "Usage: record <SESSION> [CMD [ARG ..]]"
      return 1
    fi
    name="$(awk -F '=' '/^pkg_name/ {print $2}' $1/plan.sh 2>/dev/null | sed "s/['\"]//g")"
    if [[ -z "${name:-}" ]]; then
      if [[ -f $1/habitat/plan.sh ]]; then
        name="$(awk -F '=' '/^pkg_name/ {print $2}' $1/habitat/plan.sh 2>/dev/null | sed "s/['\"]//g")"
      else
        name="unknown"
      fi
    fi
    shift
    cmd="${1:-${SHELL:-sh} -l}"; shift
    bb=${BUSYBOX:-}
    env="$($bb env \
      | $bb sed -e "s,^,'," -e "s,$,'," -e 's,0;32m,0;31m,g' \
      | $bb tr '\n' ' ')"
    log="${LOGDIR:-/src/results/logs}/${name}.$($bb date -u +%Y-%m-%d-%H%M%S).log"
    $bb mkdir -p $($bb dirname $log)
    unset BUSYBOX LOGDIR

    $bb script -c "$bb env -i $env $cmd $*" -e $log
  ); return $?
}

cd /src

PROFILE
    fi

    $bb mkdir -p $v $HAB_STUDIO_ROOT/src
    # Mount the `$SRC_PATH` under `/src` in the Studio, unless either `$NO_MOUNT`
    # or `$NO_SRC_PATH` are set
    if [ -z "${NO_MOUNT}" -a -z "${NO_SRC_PATH}" ]; then
      if ! $bb mount | $bb grep -q "on $HAB_STUDIO_ROOT/src type"; then
        $bb mount $v --bind $SRC_PATH $HAB_STUDIO_ROOT/src
      fi
    fi
  fi
}

# **Internal** Interactively enter a Studio.
enter_studio() {
  # If a non-zero sized Studio configuration is not found, exit the program.
  if [ ! -s $HAB_STUDIO_ROOT/.studio ]; then
    exit_with "Directory $HAB_STUDIO_ROOT does not appear to be a Studio, aborting" 5
  fi
  # Check if a pre-existing Studio configuration is found and use that to
  # determine the type. If no config is found, set the type to `unknown`.
  if [ -s "$studio_config" ]; then
    . "$studio_config"
    STUDIO_TYPE=$studio_type
  else
    STUDIO_TYPE=unknown
  fi

  local env="$(chroot_env "$studio_path" "$studio_enter_environment")"

  info "Entering Studio at $HAB_STUDIO_ROOT ($STUDIO_TYPE)"
  report_env_vars
  echo

  if [ -n "$VERBOSE" ]; then
    set -x
  fi

  trap cleanup_studio EXIT

  # Become the `chroot` process
  $bb chroot "$HAB_STUDIO_ROOT" \
    $studio_env_command -i $env $studio_enter_command $*
}

# **Internal** Run a build command using a Studio.
build_studio() {
  # If a non-zero sized Studio configuration is not found, exit the program.
  if [ ! -s $HAB_STUDIO_ROOT/.studio ]; then
    exit_with "Directory $HAB_STUDIO_ROOT does not appear to be a Studio, aborting" 5
  fi
  # Check if a pre-existing Studio configuration is found and use that to
  # determine the type. If no config is found, set the type to `unknown`.
  if [ -s "$studio_config" ]; then
    . "$studio_config"
    STUDIO_TYPE=$studio_type
  else
    STUDIO_TYPE=unknown
  fi

  # If a build command is not set, then this type does not support the `build`
  # subcommand and should abort.
  if [ -z "$studio_build_command" ]; then
    exit_with "Studio at $HAB_STUDIO_ROOT ($STUDIO_TYPE) does not support 'build'" 10
  fi

  local env="$(chroot_env "$studio_path" "$studio_build_environment")"

  info "Building '$*' in Studio at $HAB_STUDIO_ROOT ($STUDIO_TYPE)"
  report_env_vars

  if [ -n "$VERBOSE" ]; then
    set -x
  fi

  trap cleanup_studio EXIT

  # Run the build command in the `chroot` environment
  echo $studio_build_command $* | $bb chroot "$HAB_STUDIO_ROOT" \
    $studio_env_command -i $env $studio_run_command
}

# **Internal** Run an arbitrary command in a Studio.
run_studio() {
  # If a non-zero sized Studio configuration is not found, exit the program.
  if [ ! -s $HAB_STUDIO_ROOT/.studio ]; then
    exit_with "Directory $HAB_STUDIO_ROOT does not appear to be a Studio, aborting" 5
  fi
  # Check if a pre-existing Studio configuration is found and use that to
  # determine the type. If no config is found, set the type to `unknown`.
  if [ -s "$studio_config" ]; then
    . "$studio_config"
    STUDIO_TYPE=$studio_type
  else
    STUDIO_TYPE=unknown
  fi

  local env="$(chroot_env "$studio_path" "$studio_run_environment")"

  info "Running '$*' in Studio at $HAB_STUDIO_ROOT ($STUDIO_TYPE)"

  if [ -n "$VERBOSE" ]; then
    set -x
  fi

  trap cleanup_studio EXIT

  # Run the command in the `chroot` environment
  echo $* | $bb chroot "$HAB_STUDIO_ROOT" \
    $studio_env_command -i $env $studio_run_command
}

# **Internal** Destroy a Studio.
rm_studio() {
  # Check if a pre-existing Studio configuration is found and use that to
  # determine the type. If no config is found, set the type to `unknown`.
  if [ -s "$studio_config" ]; then
    . "$studio_config"
    STUDIO_TYPE=$studio_type
  else
    STUDIO_TYPE=unknown
  fi

  if [ -d "$HAB_STUDIO_ROOT" ]; then
    # Properly canonicalize the root path of the Studio by following all symlinks.
    HAB_STUDIO_ROOT="$($bb readlink -f $HAB_STUDIO_ROOT)"
  fi

  info "Destroying Studio at $HAB_STUDIO_ROOT ($STUDIO_TYPE)"

  trap cleanup_studio EXIT

  # Remove remaining filesystem
  $bb rm -rf $v $HAB_STUDIO_ROOT
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
    printf -- "   ${program:-unknown}: $1\n"
  else
    case "${TERM:-}" in
      *term | xterm-* | rxvt | screen | screen-*)
        printf -- "   \033[1;36m${program:-unknown}: \033[1;37m$1\033[0m\n"
        ;;
      *)
        printf -- "   ${program:-unknown}: $1\n"
        ;;
    esac
  fi
  return 0
}

# **Internal** Exit if current user is not root.
ensure_root() {
  # Early return if we are root, yay!
  if [ $($bb id -u) -eq 0 ]; then
    return
  fi

  # Otherwise, prepare to die with message formatting similar to the `hab` program.
  local msg
  local fatal

  warn_msg="Running Habitat Studio requires root or administrator privileges. \
Please retry this command as a super user or use a privilege-granting facility such as sudo."
  fatal_msg="Root or administrator permissions required to complete operation"

  if [ "${HAB_NOCOLORING:-}" = "true" ]; then
    printf -- "∅ $warn_msg\n\n✗✗✗\n✗✗✗ $fatal_msg\n✗✗✗\n"
  else
    case "${TERM:-}" in
      *term | xterm-* | rxvt | screen | screen-*)
        printf -- "\033[0;33m∅ $warn_msg\033[0m\n\n\033[0;31m✗✗✗\n✗✗✗ $fatal_msg\n✗✗✗\033[0m\n"
        ;;
      *)
        printf -- "∅ $warn_msg\n\n✗✗✗\n✗✗✗ $fatal_msg\n✗✗✗\n"
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
    printf -- "ERROR: $1\n"
  else
    case "${TERM:-}" in
      *term | xterm-* | rxvt | screen | screen-*)
        printf -- "\033[1;31mERROR: \033[1;37m$1\033[0m\n"
        ;;
      *)
        printf -- "ERROR: $1\n"
        ;;
    esac
  fi
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
  # If a Habitat config filetype ignore string is set, then propagate it
  # into the Studio's environment.
  if [ -n "${HAB_CONFIG_EXCLUDE:-}" ]; then
    env="$env HAB_CONFIG_EXCLUDE=$HAB_CONFIG_EXCLUDE"
  fi
  # If a Habitat Depot URL is set, then propagate it into the Studio's
  # environment.
  if [ -n "${HAB_DEPOT_URL:-}" ]; then
    env="$env HAB_DEPOT_URL=$HAB_DEPOT_URL"
  fi
  # If a Habitat Depot Channel is set, then propagate it into the Studio's
  # environment.
  if [ -n "${HAB_DEPOT_CHANNEL:-}" ]; then
    env="$env HAB_DEPOT_CHANNEL=$HAB_DEPOT_CHANNEL"
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
  # Used to customize the arguments to pass to an automatically launched
  # Supervisor, or to disable the automatic launching (by setting to 'false').
  if [ -n "${HAB_STUDIO_SUP:-}" ]; then
    # We want to pass a value that contains spaces, so we'll encode the spaces
    # for unpacking inside the Studio. Sorry world, but it's after 11pm.
    env="$env HAB_STUDIO_SUP=$(echo $HAB_STUDIO_SUP | $bb sed 's/ /__sp__/g')"
  fi
  # If a Habitat update strategy frequency is set, then propagate it into the
  # Studio's environment.
  if [ -n "${HAB_UPDATE_STRATEGY_FREQUENCY_MS:-}" ]; then
    env="$env HAB_UPDATE_STRATEGY_FREQUENCY_MS=$HAB_UPDATE_STRATEGY_FREQUENCY_MS"
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
    env="$env no_proxy=$(echo $no_proxy | $bb sed 's/, /,/g')"
  fi

  echo "$env"
  return 0
}

# **Internal** Prints out any important environment variables that will be used
# inside the Studio.
report_env_vars() {
  if [ -n "${HAB_CONFIG_EXCLUDE:-}" ]; then
    info "Exported: HAB_CONFIG_EXCLUDE=$HAB_CONFIG_EXCLUDE"
  fi
  if [ -n "${HAB_ORIGIN:-}" ]; then
    info "Exported: HAB_ORIGIN=$HAB_ORIGIN"
  fi
  if [ -n "${HAB_DEPOT_URL:-}" ]; then
    info "Exported: HAB_DEPOT_URL=$HAB_DEPOT_URL"
  fi
  if [ -n "${HAB_DEPOT_CHANNEL:-}" ]; then
    info "Exported: HAB_DEPOT_CHANNEL=$HAB_DEPOT_CHANNEL"
  fi
  if [ -n "${HAB_NOCOLORING:-}" ]; then
    info "Exported: HAB_NOCOLORING=$HAB_NOCOLORING"
  fi
  if [ -n "${HAB_NONINTERACTIVE:-}" ]; then
    info "Exported: HAB_NONINTERACTIVE=$HAB_NONINTERACTIVE"
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
}

# **Internal** Run when an interactive studio exits.
cleanup_studio() {
  local lock_file
  lock_file="$HAB_STUDIO_ROOT/hab/sup/default/LOCK"

  if [ -f $lock_file ]; then
    $bb kill $($bb cat $lock_file)
  fi

  # Set the verbose flag (i.e. `-v`) for any coreutils-like commands if verbose
  # mode was requested
  if [ -n "$VERBOSE" ]; then
    local v="-v"
  else
    local v=
  fi

  # Update file ownership on files under the artifact cache path using the
  # ownership of the artifact cache directory to determine the target uid and
  # gid. This is done in an effort to leave files residing in a user directory
  # not to be owned by root.
  if [ -z "${NO_MOUNT}" -a -z "${NO_ARTIFACT_PATH}" ]; then
    local artifact_path_owner
    artifact_path_owner="$($bb stat -c '%u:%g' $ARTIFACT_PATH)"
    $bb chown -R "$artifact_path_owner" "$ARTIFACT_PATH"
  fi

  # Unmount filesystems that were previously set up in, but only if they are
  # currently mounted. You know, so you can run this all day long, like, for
  # fun and stuff.

  if $bb mount | $bb grep -q "on $HAB_STUDIO_ROOT/src type"; then
    $bb umount $v -l $HAB_STUDIO_ROOT/src
  fi

  local studio_artifact_path
  studio_artifact_path="${HAB_STUDIO_ROOT}${HAB_CACHE_ARTIFACT_PATH}"
  if $bb mount | $bb grep -q "on $studio_artifact_path type"; then
    $bb umount $v -l $studio_artifact_path
  fi

  if $bb mount | $bb grep -q "on $HAB_STUDIO_ROOT/run type"; then
    $bb umount $v $HAB_STUDIO_ROOT/run
  fi

  if $bb mount | $bb grep -q "on $HAB_STUDIO_ROOT/sys type"; then
    $bb umount $v $HAB_STUDIO_ROOT/sys
  fi

  if $bb mount | $bb grep -q "on $HAB_STUDIO_ROOT/proc type"; then
    $bb umount $v $HAB_STUDIO_ROOT/proc
  fi

  if $bb mount | $bb grep -q "on $HAB_STUDIO_ROOT/dev/pts type"; then
    $bb umount $v $HAB_STUDIO_ROOT/dev/pts
  fi

  if $bb mount | $bb grep -q "on $HAB_STUDIO_ROOT/dev type"; then
    $bb umount $v -l $HAB_STUDIO_ROOT/dev
  fi

  if $bb mount | $bb grep -q "on $HAB_STUDIO_ROOT/var/run/docker.sock type"; then
    $bb umount $v -l $HAB_STUDIO_ROOT/var/run/docker.sock
  fi

  # Remove `/dev/console` device
  $bb rm $HAB_STUDIO_ROOT/dev/console

  # Remove `/dev/null` device
  $bb rm $HAB_STUDIO_ROOT/dev/null
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
hab="$libexec_path/hab"
# The current version of Habitat Studio
version='@version@'
# The author of this program
author='@author@'
# The short version of the program name which is used in logging output
program=$($bb basename $0)

ensure_root


# ## CLI Argument Parsing

# Parse command line flags and options.
while getopts ":nNa:k:r:s:t:vqVh" opt; do
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
# The artifacts cache path to be mounted into the Studio, which defaults to the
# artifact cache path.
: ${ARTIFACT_PATH:=$HAB_CACHE_ARTIFACT_PATH}
# The directory name of the Studio (which will live under `$HAB_STUDIOS_HOME`).
# It is a directory path turned into a single directory name that can be
# deterministically re-constructed on next program invocation.
dir_name="$(echo $SRC_PATH | $bb sed -e 's,^/$,root,' -e 's,^/,,' -e 's,/,--,g')"
# The base path under which all Studios are created, which defaults to
# `/hab/studios`.
: ${HAB_STUDIOS_HOME:=/hab/studios}
# The root path of the Studio, which defaults to
# `$HAB_STUDIOS_HOME/<SRC_PATH_AS_STRING>`.
: ${HAB_STUDIO_ROOT:=$HAB_STUDIOS_HOME/$dir_name}
# A collection of comma-separated keys to be copied into the Studio's key
# cache directory. If this environment variable is not set, use the value
# from `$HAB_ORIGIN` if set, otherwise, it's empty.
: ${HAB_ORIGIN_KEYS:=${HAB_ORIGIN:-}}
# The Studio configuration file which is used to determine commands to run,
# extra environment variables, etc. Note that a valid Studio will have this
# file at the root of its filesystem.
studio_config="$HAB_STUDIO_ROOT/.studio"
# The type (flavor, variant, etc.) of Studio. Such types include `default`,
# `stage1`, and `busybox` among others.
: ${STUDIO_TYPE:=}
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
    subcommand_new $*
    ;;
  rm)
    shift
    subcommand_rm $*
    ;;
  e|en|ent|ente|enter)
    shift
    subcommand_enter $*
    ;;
  b|bu|bui|buil|build)
    shift
    subcommand_build $*
    ;;
  r|ru|run)
    shift
    subcommand_run $*
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
