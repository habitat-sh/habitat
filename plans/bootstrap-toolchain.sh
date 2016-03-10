#!/bin/sh
set -eu
if [ -n "${DEBUG:-}" ]; then set -x; fi

_on_exit() {
  local exit_status=${1:-$?}
  local elapsed=$SECONDS
  elapsed=$(echo $elapsed | awk '{printf "%dm%ds", $1/60, $1%60}')
  printf -- "\n$(basename $0) run time: $elapsed\n\n"
  if [ $exit_status -ne 0 ]; then echo "Exiting on error"; fi
  exit $?
}
trap _on_exit 1 2 3 15 ERR

: ${BUILD:=./bldr-build}
BLDR_ROOT=/opt/bldr
BLDR_PKG_ROOT="$BLDR_ROOT/pkgs"
origin=chef

build() {
  local plan="${1:-}"
  shift
  if [ "${STOP_BEFORE:-}" = "$plan" ]; then
    echo "STOP_BEFORE=$STOP_BEFORE set, stopping before $plan. Cheers ;)"
    exit 0
  fi
  local db="tmp/${DB_PREFIX:-}bootstrap-toolchain.db"
  local path="$BLDR_PKG_ROOT/$origin/$plan"
  local manifest
  local ident
  local cmd
  mkdir -p $(dirname $db)
  touch $db

  if grep -q "^$origin/$plan:$*$" $db > /dev/null; then
    if ident=$(find $path -name IDENT -type f 2>&1); then
      ident="$(echo $ident | tr ' ' '\n' | sort | tail -n 1)"
      if [ ! -f "$ident" ]; then
        >&2 echo "[$plan] Missing file $ident, something is wrong, aborting"
        exit 1
      fi
      echo "[$plan] Previous build found in db $db, skipping ($(cat $ident))"
      return 0
    else
      >&2 echo "[$plan] Found in db $db but missing on disk, aborting"
      exit 2
    fi
  fi

  if [ -n "$*" ]; then
    cmd="env $* $BUILD $plan"
  else
    cmd="$BUILD $plan"
  fi
  echo "[$plan] Building with: $cmd"
  eval $cmd
  echo "[$plan] Recording build record in $db"
  echo "$origin/$plan:$*" >> $db
}

cat <<_PLANS_ | while read plan; do build $plan; done
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
