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