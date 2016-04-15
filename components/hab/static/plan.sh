source ../plan.sh

pkg_name=hab-static
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
# The result is a portible, static binary in a zero-dependency package.
pkg_deps=()
pkg_build_deps=(
  chef/musl chef/zlib-musl chef/xz-musl chef/bzip2-musl chef/libarchive-musl
  chef/openssl-musl chef/libsodium-musl
  chef/coreutils chef/cacerts chef/rust chef/gcc
)

# Set the parent directory as the "root" of this plan.
PLAN_CONTEXT=$(abspath ..)

do_prepare() {
  _common_prepare

  export rustc_target="x86_64-unknown-linux-musl"
  build_line "Setting rustc_target=$rustc_target"

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
  export SODIUM_LIB_DIR=$(pkg_path_for libsodium-musl)/lib
  export SODIUM_STATIC=true
}


# ----------------------------------------------------------------------------
# **NOTICE:** What follows are implementation details required for building a
# first-pass, "stage1" toolchain and environment. It is only used when running
# in a "stage1" Studio and can be safely ignored by almost everyone. Having
# said that, it performs a vital bootstrapping process and cannot be removed or
# significantly altered. Thank you!
# ----------------------------------------------------------------------------
if [[ "$STUDIO_TYPE" = "stage1" ]]; then
  pkg_build_deps=(
    chef/gcc chef/coreutils chef/sed chef/grep chef/diffutils chef/make chef/cacerts
    chef/rust chef/musl chef/zlib-musl chef/bzip2-musl chef/xz-musl chef/libsodium-musl
    chef/openssl-musl chef/libarchive-musl
  )
fi
