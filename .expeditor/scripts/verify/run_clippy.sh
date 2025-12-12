#!/bin/bash

set -euo pipefail

source .expeditor/scripts/verify/shared.sh

export HAB_BLDR_CHANNEL="${HAB_BLDR_CHANNEL:="base-2025"}"
toolchain=$(get_toolchain)
install_hab_pkg core/glibc core/gcc-base core/rust/"$toolchain" core/zeromq core/protobuf

RUSTFLAGS="-D warnings"
RUSTFLAGS+=" -C link-arg=-Wl,--dynamic-linker=$(hab pkg path core/glibc)/lib/ld-linux-x86-64.so.2"
export RUSTFLAGS

LD_LIBRARY_PATH="$(cat "$(hab pkg path core/gcc-base)"/LD_RUN_PATH)"
LD_LIBRARY_PATH+=":$(cat "$(hab pkg path core/zeromq)"/LD_RUN_PATH)"
export LD_LIBRARY_PATH

PATH="$(hab pkg path core/rust/"$toolchain")/bin:$PATH"
export PATH

LIBZMQ_PREFIX=$(hab pkg path core/zeromq)
export LIBZMQ_PREFIX

export PROTOC_NO_VENDOR=1
PROTOC=$(hab pkg path core/protobuf)/bin/protoc
export PROTOC

# Lints we need to work through and decide as a team whether to allow or fix
mapfile -t unexamined_lints < "$1"
# Lints we disagree with and choose to keep in our code with no warning
mapfile -t allowed_lints < "$2"
# Known failing lints we want to receive warnings for, but not fail the build
mapfile -t lints_to_fix < "$3"
# Lints we want to avoid adding even at the cost of failing the build
mapfile -t denied_lints < "$4"

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
echo "EXECUTING: cargo clippy --workspace --all-targets --no-deps -- ${clippy_args[*]}"
cargo clippy --workspace --all-targets --no-deps -- "${clippy_args[@]}"
