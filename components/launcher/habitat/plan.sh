# shellcheck disable=2154
source "$SRC_PATH/../../support/ci/builder-base-plan.sh"
pkg_name=hab-launcher
pkg_origin=core
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('Apache-2.0')
pkg_deps=(core/glibc
          core/gcc-libs
          core/libarchive
          core/libsodium
          core/openssl)
pkg_build_deps=(core/coreutils
                core/rust
                core/gcc
                core/git)
pkg_bin_dirs=(bin)
bin="hab-launch"

# shellcheck disable=2155
do_prepare() {
  do_builder_prepare

  export LIBARCHIVE_LIB_DIR=$(pkg_path_for libarchive)/lib
  export LIBARCHIVE_INCLUDE_DIR=$(pkg_path_for libarchive)/include
  export OPENSSL_LIB_DIR=$(pkg_path_for openssl)/lib
  export OPENSSL_INCLUDE_DIR=$(pkg_path_for openssl)/include
  export SODIUM_LIB_DIR=$(pkg_path_for libsodium)/lib
}

do_build() {
  pushd "$SRC_PATH" > /dev/null || exit
  cargo build "${builder_build_type#--debug}" --target="$rustc_target" --verbose
  popd > /dev/null || exit
}

do_install() {
  install -v -D "$CARGO_TARGET_DIR"/"$rustc_target"/"${builder_build_type#--}"/$bin \
    "$pkg_prefix"/bin/$bin
}
