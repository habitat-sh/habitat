#!/bin/bash

set -euo pipefail

source .expeditor/scripts/verify/shared.sh

export RUSTFLAGS="-D warnings"

toolchain=$(get_toolchain)
install_rustup
install_rust_toolchain "$toolchain"

# Install clippy
echo "--- :rust: Installing clippy"
rustup component add --toolchain "$toolchain" clippy

# TODO: these should be in a shared script?
install_hab_pkg core/zeromq core/protobuf core/patchelf core/rust/"$toolchain"

# Yes, this is terrible but we need the clippy binary to run under our glibc.
# This became an issue with the latest refresh and can likely be dropped in
# the future when rust and supporting components are build against a later
# glibc.
sudo cp "$HOME"/.rustup/toolchains/"$toolchain"-x86_64-unknown-linux-gnu/bin/cargo-clippy "$(sudo -E hab pkg path core/rust/"$toolchain")/bin"
sudo cp "$HOME"/.rustup/toolchains/"$toolchain"-x86_64-unknown-linux-gnu/bin/clippy-driver "$(sudo -E hab pkg path core/rust/"$toolchain")/bin"
sudo -E hab pkg exec core/patchelf patchelf -- --set-interpreter "$(sudo -E hab pkg path core/glibc)/lib/ld-linux-x86-64.so.2" "$(sudo -E hab pkg path core/rust/"$toolchain")/bin/clippy-driver"
sudo -E hab pkg exec core/patchelf patchelf -- --set-interpreter "$(sudo -E hab pkg path core/glibc)/lib/ld-linux-x86-64.so.2" "$(sudo -E hab pkg path core/rust/"$toolchain")/bin/cargo-clippy"

export LIBZMQ_PREFIX
LIBZMQ_PREFIX=$(sudo -E hab pkg path core/zeromq)
# now include zeromq so it exists in the runtime library path when cargo test is run
export LD_LIBRARY_PATH
LD_LIBRARY_PATH="$(sudo -E hab pkg path core/gcc-base)/lib64:$(sudo -E hab pkg path core/zeromq)/lib"
old_path=$PATH
eval "$(sudo -E hab pkg env core/rust/"$toolchain")"
export PATH=$PATH:$old_path

export PROTOC_NO_VENDOR=1
export PROTOC
PROTOC=$(sudo -E hab pkg path core/protobuf)/bin/protoc

# Lints we need to work through and decide as a team whether to allow or fix
mapfile -t unexamined_lints < "$1"

# Lints we disagree with and choose to keep in our code with no warning
mapfile -t allowed_lints < "$2"

# Known failing lints we want to receive warnings for, but not fail the build
mapfile -t lints_to_fix < "$3"

# Lints we don't expect to have in our code at all and want to avoid adding
# even at the cost of failing the build
mapfile -t denied_lints < "$4"

clippy_args=()

add_lints_to_clippy_args() {
  flag=$1
  shift
  for lint
  do
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
cargo-clippy --version
echo "Clippy rules: cargo clippy --all-targets --tests -- ${clippy_args[*]}"
cargo-clippy clippy --all-targets --tests -- "${clippy_args[@]}"
