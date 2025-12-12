#!/bin/bash

set -euo pipefail

source .expeditor/scripts/verify/shared.sh

if ${BUILDKITE:-false}; then
  toolchain=$(get_toolchain)
  sudo -E hab pkg install core/rust/"$toolchain"
  PATH="$(hab pkg path core/rust/"$toolchain")/bin:$PATH"
  export PATH
fi

RUSTFLAGS="-D warnings "
RUSTFLAGS+="-Clink-arg=-Wl,--dynamic-linker=/hab/pkgs/core/glibc/2.41/20250627111239/lib64/ld-linux-x86-64.so.2 "
RUSTFLAGS+="-Clink-arg=-Wl,-rpath,/hab/pkgs/core/gcc-base/14.3.0/20250627114133/lib64 "
readonly RUSTFLAGS
export RUSTFLAGS

install_hab_pkg core/zeromq core/protobuf

LIBZMQ_PREFIX=$(hab pkg path core/zeromq)
export LIBZMQ_PREFIX

LD_LIBRARY_PATH="$(hab pkg path core/zeromq)/lib"
export LD_LIBRARY_PATH

export PROTOC_NO_VENDOR=1
PROTOC=$(hab pkg path core/protobuf)/bin/protoc
export PROTOC

# Usage helper
usage() {
  echo "Usage: $0 [-h] [-f] <unexamined_lints> <allowed_lints> <lints_to_fix> <denied_lints>"
  echo ""
  echo "Options:"
  echo "  -h   Show this help and exit"
  echo "  -f   Run clippy with --fix to automatically apply fixes"
}

# Parse options
fix_mode=false
while getopts ":hf" opt; do
  case "$opt" in
  h)
    usage
    exit 0
    ;;
  f)
    fix_mode=true
    ;;
  :)
    echo "Option -$OPTARG requires an argument" >&2
    usage
    exit 2
    ;;
  \?)
    echo "Invalid option: -$OPTARG" >&2
    usage
    exit 2
    ;;
  esac
done
shift $((OPTIND - 1))

# Validate required positional arguments
if [ $# -ne 4 ]; then
  echo "Error: Expected 4 lint list files, got $#" >&2
  usage
  exit 2
fi

# Lints we need to work through and decide as a team whether to allow or fix
mapfile -t unexamined_lints <"$1"
# Lints we disagree with and choose to keep in our code with no warning
mapfile -t allowed_lints <"$2"
# Known failing lints we want to receive warnings for, but not fail the build
mapfile -t lints_to_fix <"$3"
# Lints we want to avoid adding even at the cost of failing the build
mapfile -t denied_lints <"$4"

clippy_args=()

add_lints_to_clippy_args() {
  flag=$1
  shift
  for lint; do
    clippy_args+=("$flag" "${lint}")
  done
}

set +u # See https://stackoverflow.com/questions/7577052/bash-empty-array-expansion-with-set-u/39687362#39687362
add_lints_to_clippy_args -A "${unexamined_lints[@]}"
add_lints_to_clippy_args -A "${allowed_lints[@]}"
add_lints_to_clippy_args -W "${lints_to_fix[@]}"
add_lints_to_clippy_args -D "${denied_lints[@]}"
set -u

echo "--- Running clippy!"
cargo version
cargo clippy --version
if [ "$fix_mode" = true ]; then
  echo "EXECUTING: cargo clippy --fix --allow-staged -- ${clippy_args[*]}"
  # --fix implies --all-targets and --no-deps
  cargo clippy --fix --allow-staged -- "${clippy_args[@]}"
else
  echo "EXECUTING: cargo clippy --all-targets --no-deps -- ${clippy_args[*]}"
  cargo clippy --all-targets --no-deps -- "${clippy_args[@]}"
fi
