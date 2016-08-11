PLAN_CONTEXT=$(dirname $PLAN_CONTEXT) source ../plan.sh

pkg_name=hab-dynamic
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_deps=(
  core/glibc core/zlib core/xz core/bzip2 core/libarchive
  core/openssl core/libsodium core/gcc-libs
)
pkg_build_deps=(core/coreutils core/cargo-nightly core/rust core/gcc)
pkg_bin_dirs=(bin)

do_begin() {
  # Set the parent directory as the "root" of this plan.
  PLAN_CONTEXT=$(abspath ..)
}

do_prepare() {
  _common_prepare

  export rustc_target="x86_64-unknown-linux-gnu"
  build_line "Setting rustc_target=$rustc_target"

  export LIBARCHIVE_LIB_DIR=$(pkg_path_for libarchive)/lib
  export LIBARCHIVE_INCLUDE_DIR=$(pkg_path_for libarchive)/include
  export OPENSSL_LIB_DIR=$(pkg_path_for openssl)/lib
  export OPENSSL_INCLUDE_DIR=$(pkg_path_for openssl)/include
  export SODIUM_LIB_DIR=$(pkg_path_for libsodium)/lib
}
