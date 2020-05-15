# shellcheck disable=2154
pkg_name=hab-launcher
pkg_origin=core
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('Apache-2.0')
pkg_deps=(core/glibc
          core/gcc-libs
          core/openssl)
pkg_build_deps=(core/coreutils
                core/rust/"$(cat "$SRC_PATH/../../rust-toolchain")"
                core/gcc
                core/git
                core/protobuf)
pkg_bin_dirs=(bin)
bin="hab-launch"

# Use the number of commits from the start of this repository
# to the current HEAD as the version for our pkg_version
pkg_version() {
  git rev-list "$(git rev-parse HEAD)" --count
}

do_before() {
  update_pkg_version
}

# shellcheck disable=2155
do_prepare() {
  export cargo_build_mode="${cargo_build_mode:---release}"
  # Can be either `--release` or `--debug` to determine cargo build strategy
  build_line "Building artifacts with \`${cargo_build_mode#--}' mode"

  export rustc_target="x86_64-unknown-linux-gnu"
  build_line "Setting rustc_target=$rustc_target"

  # Used by Cargo to use a pristine, isolated directory for all compilation
  export CARGO_TARGET_DIR="$HAB_CACHE_SRC_PATH/$pkg_dirname"
  build_line "Setting CARGO_TARGET_DIR=$CARGO_TARGET_DIR"
  
  export PLAN_VERSION="${pkg_version}/${pkg_release}"
  build_line "Setting PLAN_VERSION=$PLAN_VERSION"

  export OPENSSL_LIB_DIR=$(pkg_path_for openssl)/lib
  export OPENSSL_INCLUDE_DIR=$(pkg_path_for openssl)/include

  # Prost (our Rust protobuf library) embeds a `protoc` binary, but
  # it's dynamically linked, which means it won't work in a
  # Studio. Prost does allow us to override that, though, so we can
  # just use our Habitat package by setting these two environment
  # variables.
  export PROTOC="$(pkg_path_for protobuf)/bin/protoc"
  export PROTOC_INCLUDE="$(pkg_path_for protobuf)/include"

  # Used to set the active package target for the binaries at build time
  export PLAN_PACKAGE_TARGET="$pkg_target"
  build_line "Setting PLAN_PACKAGE_TARGET=$PLAN_PACKAGE_TARGET"
}

do_build() {
  pushd "$SRC_PATH" > /dev/null || exit
  cargo build "${cargo_build_mode#--debug}" --target="$rustc_target" --verbose
  popd > /dev/null || exit
}

do_install() {
  install -v -D "$CARGO_TARGET_DIR"/"$rustc_target"/"${cargo_build_mode#--}"/$bin \
    "$pkg_prefix"/bin/$bin
}

do_strip() {
  strip --strip-debug "$pkg_prefix"/bin/$bin
}
