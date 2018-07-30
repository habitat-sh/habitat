#!/bin/bash

# TESTING CHANGES
# Documentation on testing local changes to this lives here:
# https://github.com/habitat-sh/habitat/blob/master/BUILDING.md#testing-changes

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
  PS1='[\#]'${HAB_STUDIO_BINARY+[HAB_STUDIO_BINARY]}'['${STUDIO_TYPE:-unknown}':\w:$(echo -n $?)]\$ '
else
  case "${TERM:-}" in
  *term | xterm-* | rxvt | screen | screen-*)
    PS1='\[\e[0;32m\][\[\e[0;36m\]\#\[\e[0;32m\]]${HAB_STUDIO_BINARY+[\[\e[1;31m\]HAB_STUDIO_BINARY\[\e[0m\]]}['${STUDIO_TYPE:-unknown}':\[\e[0;35m\]\w\[\e[0;32m\]:\[\e[1;37m\]`echo -n $?`\[\e[0;32m\]]\$\[\e[0m\] '
    ;;
  *)
    PS1='[\#]'${HAB_STUDIO_BINARY+[HAB_STUDIO_BINARY]}'['${STUDIO_TYPE:-unknown}':\w:$(echo -n $?)]\$ '
    ;;
  esac
fi

record() {
  (if [ -n "${DEBUG:-}" ]; then set -x; fi; unset DEBUG
    if [ -z "${1:-}" ]; then
      >&2 echo "Usage: record <SESSION> [CMD [ARG ..]]"
      return 1
    fi
    for plan_dir in "$1" "$1/habitat"; do
      if [ -f "$plan_dir/plan.sh" ]; then
        # shellcheck disable=1090,2154
        name=$(. "$plan_dir/plan.sh" 2>/dev/null && echo "$pkg_name")
        break
      fi
    done
    : "${name:=unknown}"
    shift
    cmd="${1:-${SHELL:-sh} -l}"; shift
    bb=${BUSYBOX:-}
    env="$($bb env \
      | $bb sed -e "s,^,'," -e "s,$,'," -e 's,0;32m,0;31m,g' \
      | $bb tr '\n' ' ')"
    log="${LOGDIR:-/src/results/logs}/${name}.$($bb date -u +%Y-%m-%d-%H%M%S).log"
    $bb mkdir -p "$($bb dirname "$log")"
    $bb touch "$log"
    if [[ "$log" =~ ^/src/results/logs/.* ]]; then
      ownership=$($bb stat -c '%u:%g' /src)
      $bb chown -R "$ownership" "/src/results" || true
    fi
    unset BUSYBOX LOGDIR name ownership

    $bb script -c "$bb env -i $env $cmd $*" -e "$log"
  ); return $?
}

_record_build() {
  build_command_name=$1
  plan_context=$2
  session=$plan_context
  record "$session" "$build_command_name" "$plan_context"
}

cd /src || { echo "Setup failed; exiting studio!"; exit 1; }
