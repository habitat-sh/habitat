#!/bin/sh
#
# # Usage
#
# ```sh
# $ hab-studio [FLAGS] [OPTIONS] <SUBCOMMAND> [ARG ...]
# ```
#
# See the `print_help()` function below for complete usage instructions.

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
#   mkdir -p "$v" "$HAB_STUDIO_ROOT"/dev
# Results in: mkdir: can't create directory '': No such file or directory
set_v_flag() {
  if [ -n "${VERBOSE:-}" ]; then
    v="-v"
  else
    v=
  fi
}

## Help/Usage functions

# **Internal** Prints help and usage information. Straight forward, no?
print_help() {
  echo "$program $version

$author

Habitat Studios - Plan for success!

USAGE:
        $program [FLAGS] [OPTIONS] <SUBCOMMAND> [ARG ..]

COMMON FLAGS:
    -h  Prints this message
    -q  Prints less output for better use in scripts
    -v  Prints more verbose output
    -V  Prints version information

COMMON OPTIONS:
    -a <ARTIFACT_PATH>    Sets the source artifact cache path (default: /hab/cache/artifacts)
    -c <CERT_PATH>        Sets the SSL certs cache path (default: /hab/cache/ssl)
    -f <REFRESH_CHANNEL>  Sets the channel used to retrieve plan dpendencies for Chef
                          supported origins (default: base)
    -k <HAB_ORIGIN_KEYS>  Installs secret origin keys (default:\$HAB_ORIGIN )
    -r <HAB_STUDIO_ROOT>  Sets a Studio root (default: /hab/studios/<DIR_NAME>)
    -s <SRC_PATH>         Sets the source path (default: \$PWD)
    -t <STUDIO_TYPE>      Sets a Studio type when creating (default: default)
                          Valid types: [default bootstrap]

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
    CERT_PATH              Sets the SSL cert cache path (\`-c' option overrides)
    HAB_NOCOLORING         Disables text coloring mode despite TERM capabilities
    HAB_NONINTERACTIVE     Disables interactive progress bars despite tty
    HAB_LICENSE            Set to 'accept' or 'accept-no-persist' to accept the Habitat license
    HAB_ORIGIN             Propagates this variable into any studios
    HAB_ORIGIN_KEYS        Installs secret keys (\`-k' option overrides)
    HAB_STUDIO_NOSTUDIORC  Disables sourcing a \`.studiorc' in \`studio enter'
    HAB_STUDIO_ROOT        Sets a Studio root (\`-r' option overrides)
    HAB_STUDIO_SUP         Sets args for a Supervisor in \`studio enter'
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

    # Run a command in the slim Studio, showing only the command output
    $program -q ls -l /
"
}

print_build_help() {
  echo "${program}-build $version

$author

Habitat Studios - execute a build using a Studio

USAGE:
        $program [COMMON_FLAGS] [COMMON_OPTIONS] build [PLAN_DIR]

EXAMPLES:

    # Build a Redis plan
    $program build plans/redis
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
  unset | default)
    # Set the default/unset type
    STUDIO_TYPE="default"
    ;;
  bootstrap)
    # Confirmed valid types
    ;;
  *)
    # Everything else is invalid
    exit_with "Invalid Studio type: $STUDIO_TYPE" 2
    ;;
  esac

  # Properly canonicalize the root path of the Studio by following all symlinks.
  $mkdir_cmd -p "$HAB_STUDIO_ROOT"
  HAB_STUDIO_ROOT="$($readlink_cmd -f "$HAB_STUDIO_ROOT")"

  info "Creating Studio at $HAB_STUDIO_ROOT ($STUDIO_TYPE)"

  # Load the appropriate type strategy to complete the setup
  if [ -n "${HAB_STUDIO_BINARY:-}" ]; then
    # shellcheck disable=SC2154
    studio_type_dir="$studio_binary_libexec_path"
  else
    studio_type_dir="$libexec_path"
  fi

  # shellcheck disable=1090
  . "$studio_type_dir/hab-studio-type-darwin-${STUDIO_TYPE}.sh"

  # Invoke the type's implementation
  finish_setup

  # Add a Studio configuration file at the root of the filesystem
  $cat_cmd <<EOF >"$studio_config"
  studio_type="$studio_type"
  studio_env_command="${studio_env_command:?}"
  studio_enter_environment="${studio_enter_environment?}"
  studio_enter_command="${studio_enter_command:?}"
  studio_build_environment="${studio_build_environment?}"
  studio_build_command="${studio_build_command?}"
  studio_run_environment="${studio_run_environment?}"
EOF

  # If `/etc/profile` is not present, create a minimal version with convenient
  # helper functions. "bare" studio doesn't need an /etc/profile
  pfile="$HAB_STUDIO_ROOT/etc/profile"
  if [ ! -f "$pfile" ] || ! "$grep_cmd" -q '^record() {$' "$pfile"; then
    if [ -n "$VERBOSE" ]; then
      echo "> Creating /etc/profile"
    fi

    if [ -n "${HAB_STUDIO_BINARY:-}" ]; then
      studio_profile_dir="$studio_binary_libexec_path"
    else
      studio_profile_dir="$libexec_path"
    fi
    "$cat_cmd" "$studio_profile_dir/hab-studio-darwin-profile.sh" >>"$pfile"
  fi

}

# **Internal** Interactively enter a Studio.
enter_studio() {
  exit_if_no_studio_config
  source_studio_type_config

  sandbox_env=$(build_sandbox_env "$studio_enter_environment")
  work_dir=$($pwd_cmd)
  work_dir=$($readlink_cmd -f "$work_dir")
  sandbox_profile_path="$libexec_path/darwin-sandbox.sb"

  info "Entering Studio at $HAB_STUDIO_ROOT ($STUDIO_TYPE)"
  report_env_vars

  if [ -n "$VERBOSE" ]; then
    set -x
  fi

  # shellcheck disable=2086
  $studio_env_command -i \
    $sandbox_env \
    "$sandbox_exec_cmd" -f "$sandbox_profile_path" \
    -DWORK_DIR="$work_dir" \
    -DSTUDIO_DIR="$HAB_STUDIO_ROOT" \
    -DSTUDIO_HAB="$libexec_path/hab" \
    $studio_enter_command "$@"
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

  sandbox_env=$(build_sandbox_env "$studio_build_environment")
  work_dir=$($pwd_cmd)
  work_dir=$($readlink_cmd -f "$work_dir")
  sandbox_profile_path="$libexec_path/darwin-sandbox.sb"

  info "Building '$*' in Studio at $HAB_STUDIO_ROOT ($STUDIO_TYPE)"
  report_env_vars

  if [ -n "$VERBOSE" ]; then
    set -x
  fi

  # Run the build command in the `sandbox` environment
  # shellcheck disable=2086
  $studio_env_command -i \
    $sandbox_env \
    "$sandbox_exec_cmd" -f "$sandbox_profile_path" \
    -DWORK_DIR="$work_dir" \
    -DSTUDIO_DIR="$HAB_STUDIO_ROOT" \
    -DSTUDIO_HAB="$libexec_path/hab" \
    $studio_build_command "$@"
}

# **Internal** Destroy a Studio.
rm_studio() {

  source_studio_type_config

  if [ -d "$HAB_STUDIO_ROOT" ]; then
    # Properly canonicalize the root path of the Studio by following all symlinks.
    HAB_STUDIO_ROOT="$($readlink_cmd -f "$HAB_STUDIO_ROOT")"
  fi

  info "Destroying Studio at $HAB_STUDIO_ROOT ($STUDIO_TYPE)"

  cleanup_studio
  $rm_cmd -rf "$HAB_STUDIO_ROOT"
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
  if [ "$($id_cmd -u)" -eq 0 ]; then
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
  $env_cmd | $awk_cmd -F '=' '/^HAB_STUDIO_SECRET_/ {gsub(/HAB_STUDIO_SECRET_/, ""); print}'
}

build_sandbox_env() {
  extra_env="$1"

  sandbox_env="LC_ALL=POSIX TERM=${TERM:-} PATH=${HAB_STUDIO_ROOT}${HAB_ROOT_PATH}/bin:/usr/bin:/bin:/usr/sbin"

  # Create temporary directory for usage inside the sandbox during builds
  $mkdir_cmd -p /hab/cache/tmp
  TMPDIR=$($mktemp_cmd -p /hab/cache/tmp -d)
  sandbox_env="$sandbox_env TMPDIR=$TMPDIR"

  # Add `STUDIO_TYPE` to the environment
  sandbox_env="$sandbox_env STUDIO_TYPE=$STUDIO_TYPE"
  # Add any additional environment variables from the Studio config, based on
  # type
  if [ -n "$extra_env" ]; then
    sandbox_env="$sandbox_env $extra_env"
  fi
  if [ -n "${SUDO_USER:-}" ]; then
    sandbox_env="$sandbox_env SUDO_USER=$SUDO_USER"
  fi
  # If a Habitat config filetype ignore string is set, then propagate it
  # into the Studio's sandbox environment.
  if [ -n "${HAB_CONFIG_EXCLUDE:-}" ]; then
    sandbox_env="$sandbox_env HAB_CONFIG_EXCLUDE=$HAB_CONFIG_EXCLUDE"
  fi
  # If a Habitat Auth Token is set, then propagate it into the Studio's
  # sandbox_environment.
  if [ -n "${HAB_AUTH_TOKEN:-}" ]; then
    sandbox_env="$sandbox_env HAB_AUTH_TOKEN=$HAB_AUTH_TOKEN"
  fi
  # If a Habitat Builder URL is set, then propagate it into the Studio's
  # sandbox_environment.
  if [ -n "${HAB_BLDR_URL:-}" ]; then
    sandbox_env="$sandbox_env HAB_BLDR_URL=$HAB_BLDR_URL"
  fi
  # If a Habitat Depot Channel is set, then propagate it into the Studio's
  # sandbox_environment.
  if [ -n "${HAB_BLDR_CHANNEL:-}" ]; then
    sandbox_env="$sandbox_env HAB_BLDR_CHANNEL=$HAB_BLDR_CHANNEL"
  fi
  # If a Habitat refresh Channel is set, then propagate it into the Studio's
  # environment.
  if [ -n "${HAB_REFRESH_CHANNEL:-}" ]; then
    sandbox_env="$sandbox_env HAB_REFRESH_CHANNEL=$HAB_REFRESH_CHANNEL"
  fi
  # If a no coloring environment variable is set, then propagate it into the Studio's
  # sandbox environment.
  if [ -n "${HAB_NOCOLORING:-}" ]; then
    sandbox_env="$sandbox_env HAB_NOCOLORING=$HAB_NOCOLORING"
  fi
  # If a noninteractive environment variable is set, then propagate it into the Studio's
  # sandbox environment.
  if [ -n "${HAB_NONINTERACTIVE:-}" ]; then
    sandbox_env="$sandbox_env HAB_NONINTERACTIVE=$HAB_NONINTERACTIVE"
  fi
  # If the hab license is set, then propagate that into the Studio's environment
  if [ -f "/hab/accepted-licenses/habitat" ] || [ -f "$HOME/.hab/accepted-licenses/habitat" ]; then
    sandbox_env="$sandbox_env HAB_LICENSE=accept-no-persist"
  elif [ -n "${HAB_LICENSE:-}" ]; then
    sandbox_env="$sandbox_env HAB_LICENSE=$HAB_LICENSE"
  fi
  # If a Habitat origin name is set, then propagate it into the Studio's
  # sandbox_environment.
  if [ -n "${HAB_ORIGIN:-}" ]; then
    sandbox_env="$sandbox_env HAB_ORIGIN=$HAB_ORIGIN"
  fi
  # If a skip .studiorc environment variable is set, then propagate it into the
  # Studio's environment.
  if [ -n "${HAB_STUDIO_NOSTUDIORC:-}" ]; then
    env="$env HAB_STUDIO_NOSTUDIORC=$HAB_STUDIO_NOSTUDIORC"
  fi
  # If a Habitat output path is set, then propagate it into the Studio's
  # sandbox_environment.
  if [ -n "${HAB_OUTPUT_PATH-}" ]; then
    sandbox_env="$sandbox_env HAB_OUTPUT_PATH=$HAB_OUTPUT_PATH"
  fi
  # If DO_CHECK is set, then propagate it into the Studio's sandbox_environment.
  if [ -n "${DO_CHECK:-}" ]; then
    sandbox_env="$sandbox_env DO_CHECK=$DO_CHECK"
  fi
  sandbox_env="$sandbox_env $(load_secrets)"

  echo "$sandbox_env"
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
  if [ -n "${HAB_REFRESH_CHANNEL:-}" ]; then
    info "Exported: HAB_REFRESH_CHANNEL=$HAB_REFRESH_CHANNEL"
  fi
  for secret_name in $(load_secrets | $cut_cmd -d = -f 1); do
    info "Exported: $secret_name=[redacted]"
  done
}

# **Internal** Run when an interactive studio exits.
cleanup_studio() {
  chown_artifacts
  chown_certs
}

# **Internal** Updates file ownership on files under the artifact cache path
# using the ownership of the artifact cache directory to determine the target
# uid and gid. This is done in an effort to leave files residing in a user
# directory not to be owned by root.
chown_artifacts() {
  if [ -d "$ARTIFACT_PATH" ]; then
    if [ "$stat_variant" = "bsd" ]; then
      artifact_path_owner="$("$stat_cmd" -f '%Su:%g' "$ARTIFACT_PATH")" || echo "stat on $ARTIFACT_PATH failed with $?"
    else
      artifact_path_owner="$("$stat_cmd" -c '%u:%g' "$ARTIFACT_PATH")" || echo "stat on $ARTIFACT_PATH failed with $?"
    fi
    "$chown_cmd" -R "$artifact_path_owner" "$ARTIFACT_PATH"
  fi
}

# **Internal** Updates file ownership on files under the SSL cert cache path
# using the ownership of the SSL cert cache directory to determine the target
# uid and gid. This is done in an effort to leave files residing in a user
# directory not to be owned by root.
chown_certs() {
  if [ -d "$CERT_PATH" ]; then
    if [ "$stat_variant" = "bsd" ]; then
      cert_path_owner="$($stat_cmd -f '%Su:%g' "$CERT_PATH")" || echo "stat on $CERT_PATH failed with $?"
    else
      cert_path_owner="$($stat_cmd -c '%u:%g' "$CERT_PATH")" || echo "stat on $CERT_PATH failed with $?"
    fi
    "$chown_cmd" -R "$cert_path_owner" "$CERT_PATH"
  fi
}

find_system_cmds() {
  pwd_cmd="$(command -v pwd)"
  env_cmd="$(command -v env)"
  sandbox_exec_cmd="/usr/bin/sandbox-exec"
  mkdir_cmd="$(command -v mkdir)"
  cat_cmd="$(command -v cat)"
  grep_cmd="$(command -v grep)"
  awk_cmd="$(command -v awk)"
  cut_cmd="$(command -v cut)"
  chown_cmd="$(command -v chown)"
  chmod_cmd="$(command -v chmod)"
  mktemp_cmd="$(command -v mktemp)"
  stat_cmd=$(command -v stat)
  if $stat_cmd -f '%Su:%g' . 2>/dev/null 1>/dev/null; then
    stat_variant="bsd"
  elif $stat_cmd -c '%u:%g' . 2>/dev/null 1>/dev/null; then
    stat_variant="gnu"
  else
    exit_with "Failed to determine stat variant, we require GNU or BSD stat to determine user and group owners of files; aborting" 1
  fi
  sed_cmd="$(command -v sed)"
  rm_cmd="$(command -v rm)"
  readlink_cmd="$(command -v readlink)"
  system_hab_cmd="$(command -v hab)"
  id_cmd="$(command -v id)"
}

# **Internal** Sets the `$libexec_path` variable, which is the absolute path to
# the `libexec/` directory for this software.
set_libexec_path() {
  script_path="${0%/*}"
  script_path="$(
    cd "$script_path" || exit
    $pwd_cmd
  )/${0##*/}"
  script_path=$($readlink_cmd -f "$script_path")
  script_dir="${script_path%/*}"
  libexec_path="$($readlink_cmd -f "$script_dir/../libexec")"
  studio_binary_libexec_path=$libexec_path
  return 0
}

# # Main Flow

# Finds absoulte paths to the minimal set of system commands we require
find_system_cmds
# Set the `$libexec_path` variable containing an absolute path to `../libexec`
# from this program. This directory contains Studio type definitions and the
# `busybox` binary which is used for all shell out commands.
set_libexec_path
# Finally, unset `PATH` so there is zero chance we're going to rely on the
# operating system's commands by mistake.
unset PATH

## Default variables

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

# The default root path for SSL certs
HAB_CACHE_CERT_PATH=$HAB_ROOT_PATH/cache/ssl

# The exit code for a coding error that manifests at runtime
ERR_RUNTIME_CODING_ERROR=70

# The current version of Habitat Studio
: "${version:=@version@}"
# The author of this program
author='@author@'
# The short version of the program name which is used in logging output
program="${0##*/}"

ensure_root

## CLI Argument Parsing

# Parse command line flags and options.
while getopts ":a:c:f:k:r:s:t:vqVh" opt; do
  case $opt in
  a)
    ARTIFACT_PATH=$OPTARG
    ;;
  c)
    CERT_PATH=$OPTARG
    ;;
  f)
    export HAB_REFRESH_CHANNEL="$OPTARG"
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
: "${SRC_PATH:=$($pwd_cmd)}"
# The artifacts cache path to be mounted into the Studio, which defaults to the
# artifact cache path.
: "${ARTIFACT_PATH:=$HAB_CACHE_ARTIFACT_PATH}"
# The SSL cert cache path to be mounted into the Studio, which defaults to the
# cert cache path.
: "${CERT_PATH:=$HAB_CACHE_CERT_PATH}"
# The directory name of the Studio (which will live under `$HAB_STUDIOS_HOME`).
# It is a directory path turned into a single directory name that can be
# deterministically re-constructed on next program invocation.
dir_name="$(echo "$SRC_PATH" | $sed_cmd -e 's,^/$,root,' -e 's,^/,,' -e 's,/,--,g' -e 's, ,-,g')"
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
# Whether or not more verbose output has been requested. An unset or empty
# value means it is set to false and any other value is considered set or true.
: "${VERBOSE:=}"
set_v_flag
# Whether or not less output has been requested. An unset or empty value means
# it is set to false and any other value is considered set or true.
: "${QUIET:=}"

export VERBOSE QUIET

# Next, determine the subcommand and delegate its behavior to the appropriate
# function. Note that the multiple word fragments for each case result in a
# "fuzzy matching" behavior, meaning that `studio e` is equivalent to `studio
# enter`.
case ${1:-} in
n | ne | new)
  shift
  subcommand_new "$@"
  ;;
rm)
  shift
  subcommand_rm "$@"
  ;;
e | en | ent | ente | enter)
  shift
  subcommand_enter "$@"
  ;;
b | bu | bui | buil | build)
  shift
  subcommand_build "$@"
  ;;
r | ru | run)
  shift
  subcommand_run "$@"
  ;;
v | ve | ver | vers | versi | versio | version)
  echo "$program $version"
  exit 0
  ;;
h | he | hel | help)
  print_help
  exit 0
  ;;
*)
  print_help
  exit_with "Invalid argument: ${1:-}" 1
  ;;
esac
