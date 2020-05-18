# shellcheck disable=2154
pkg_name=hab-launcher
pkg_origin=core
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('Apache-2.0')
pkg_build_deps=(
    core/musl
    core/zlib-musl
    core/xz-musl
    core/bzip2-musl
    core/libarchive-musl
    core/openssl-musl

    core/coreutils
    core/rust/"$(cat "$SRC_PATH/../../rust-toolchain")"
    core/gcc
    core/git
    core/protobuf
)
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

  export rustc_target="x86_64-unknown-linux-musl"
  build_line "Setting rustc_target=$rustc_target"

  # Used by Cargo to use a pristine, isolated directory for all compilation
  export CARGO_TARGET_DIR="$HAB_CACHE_SRC_PATH/$pkg_dirname"
  build_line "Setting CARGO_TARGET_DIR=$CARGO_TARGET_DIR"

  export PLAN_VERSION="${pkg_version}/${pkg_release}"
  build_line "Setting PLAN_VERSION=$PLAN_VERSION"

  la_ldflags="-L$(pkg_path_for zlib-musl)/lib -lz"
  la_ldflags="$la_ldflags -L$(pkg_path_for xz-musl)/lib -llzma"
  la_ldflags="$la_ldflags -L$(pkg_path_for bzip2-musl)/lib -lbz2"
  la_ldflags="$la_ldflags -L$(pkg_path_for openssl-musl)/lib -lssl -lcrypto"

  export LIBARCHIVE_LIB_DIR=$(pkg_path_for libarchive-musl)/lib
  export LIBARCHIVE_INCLUDE_DIR=$(pkg_path_for libarchive-musl)/include
  export LIBARCHIVE_LDFLAGS="$la_ldflags"
  export LIBARCHIVE_STATIC=true
  export OPENSSL_LIB_DIR=$(pkg_path_for openssl-musl)/lib
  export OPENSSL_INCLUDE_DIR=$(pkg_path_for openssl-musl)/include
  export OPENSSL_STATIC=true

  # Used to find libgcc_s.so.1 when compiling `build.rs` in
  # dependencies (`log`, in particular). Since this used only at build
  # time, we will use the version found in the gcc package proper--it
  # won't find its way into the final binaries.
  export LD_LIBRARY_PATH=$(pkg_path_for gcc)/lib
  build_line "Setting LD_LIBRARY_PATH=$LD_LIBRARY_PATH"

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
