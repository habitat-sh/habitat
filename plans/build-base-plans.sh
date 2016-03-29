#!/bin/sh
#
# # Usage
#
# ```sh
# $ build-base-plans.sh
# ```
#
# # Synopsis
#
# Builds a set of foundational base Plans comprising a fully bootstrapped Bldr
# environment which can be used to build additional software. The build order
# is very similiar to the Linux From Scratch project, and not by accident.
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

# ## Default variables

# The build command to execute. Defaults to `build`, but could be overridden if
# a full path to `bldr-build` is required.
: ${BUILD:=build}
# The root of the Bldr directory tree.
BLDR_ROOT=/opt/bldr
# Location containing installed packages.
BLDR_PKG_ROOT="$BLDR_ROOT/pkgs"
# The default package origin which was used to in the base Plans
origin=chef


# ## Private/Internal helper functions

# **Internal** Handles exiting the program on signals. Takes either an argument
# with the status code, or uses the last command's status code.
#
# For example, the following would both exit with a status of 1:
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
_on_exit() {
  local exit_status=${1:-$?}
  local elapsed=$SECONDS
  elapsed=$(echo $elapsed | awk '{printf "%dm%ds", $1/60, $1%60}')
  printf -- "\n$(basename $0) run time: $elapsed\n\n"
  if [ $exit_status -ne 0 ]; then echo "Exiting on error"; fi
  exit $?
}
# Call the `on_exit()` function above on the following signals:
#
# * HUP (1)
# * INT (2)
# * QUIT (3)
# * TERM (15)
# * ERR - when a shell command raises an error. Useful for `set -e; set -E`.
trap _on_exit 1 2 3 15 ERR

# Executes a build on a Plan, assuming that it has not already been built by a
# previous execution of this program. A very simple, plaintext database is
# maintained to track every Plan that has successfully completed so that if a
# Plan in the middle fails, a developer need only fix the failing Plan, and
# re-run the program--not needed to start from sqaure one. The database is
# `tmp/build-base-plans.db` by default so deleting this file simply removes its
# build "memory".
#
# ```sh
# _build gcc
# _build coreutils EXTRA=vars FOR=command
# ```
_build() {
  local plan="${1:-}"
  shift
  # If the `$STOP_BEFORE` environment variable is set, and its value is the
  # desired Plan, then we'll stop. This is a convenient way to build up to an
  # interesting Plan without steamrolling right over it.
  if [ "${STOP_BEFORE:-}" = "$plan" ]; then
    echo "STOP_BEFORE=$STOP_BEFORE set, stopping before $plan. Cheers ;)"
    exit 0
  fi
  local db="tmp/${DB_PREFIX:-}build-base-plans.db"
  local path="$BLDR_PKG_ROOT/$origin/$plan"
  local manifest
  local ident
  local cmd
  mkdir -p $(dirname $db)
  touch $db

  # Check if the requested Plan exists in the database, meaning that this
  # program has previously built it.
  if grep -q "^$origin/$plan:$*$" $db > /dev/null; then
    # If a fully extracted/installed package exists on disk under
    # `$BLDR_PKG_ROOT`. We're using the `IDENT` metadata file as a sentinel
    # file stand-in for the package.
    if ident=$(find $path -name IDENT -type f 2>&1); then
      ident="$(echo $ident | tr ' ' '\n' | sort | tail -n 1)"
      # If the package's `IDENT` file is missing, something has gone wrong, die
      # early.
      if [ ! -f "$ident" ]; then
        >&2 echo "[$plan] Missing file $ident, something is wrong, aborting"
        exit 1
      fi
      # If all else is good, we should be able to count on this previsouly
      # built and installed package, so we will early return from this
      # function.
      echo "[$plan] Previous build found in db $db, skipping ($(cat $ident))"
      return 0
    else
      # If the entry exists in the database, but we can't find it installed on
      # disk, something is up and so we'll die early.
      >&2 echo "[$plan] Found in db $db but missing on disk, aborting"
      exit 2
    fi
  fi

  # If extra args are passed to this function, we will treat them all as
  # environment variables.
  if [ -n "$*" ]; then
    cmd="env $* $BUILD $plan"
  else
    cmd="$BUILD $plan"
  fi
  echo "[$plan] Building with: $cmd"
  eval $cmd
  # Record the successful build into our simple database
  echo "[$plan] Recording build record in $db"
  echo "$origin/$plan:$*" >> $db
}


# # Main Flow

# Read a list of tokens that are directories containing a `plan.sh` file. For
# each token, invoke the `_build` function and pass the while line in. Simple,
# no?
cat <<_PLANS_ | while read plan; do _build $plan; done
  linux-headers
  glibc
  zlib
  file
  binutils
  m4
  gmp
  mpfr
  libmpc
  gcc
  patchelf FIRST_PASS=true
  gcc-libs
  patchelf
  bzip2
  pkg-config
  ncurses
  attr
  acl
  libcap
  sed
  shadow
  psmisc
  procps-ng
  coreutils
  bison
  flex
  pcre
  grep
  readline
  bash
  bc
  tar
  gawk
  libtool
  gdbm
  expat
  db
  inetutils
  iana-etc
  less
  perl
  diffutils
  autoconf
  automake
  findutils
  xz
  gettext
  gzip
  make
  patch
  texinfo
  util-linux
  tcl
  expect
  dejagnu
  check
  libidn
  cacerts
  openssl
  wget
  gnupg
  bash-static
  coreutils-static
  linux-headers-musl
  musl
  busybox-static
  gnupg-static
  jq-static
  wget-static
  bpm
  build
  vim
  libbsd
  clens
  mg
  backline
  bldr-studio
_PLANS_

_on_exit 0
