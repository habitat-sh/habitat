source ../plan.sh

pkg_name=hab-static
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
# The result is a portible, static binary in a zero-dependency package.
pkg_deps=()
pkg_build_deps=(
  core/musl core/zlib-musl core/xz-musl core/bzip2-musl core/libarchive-musl
  core/openssl-musl core/libsodium-musl
  core/coreutils core/cacerts core/rust core/gcc
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
    core/gcc core/coreutils core/sed core/grep core/diffutils core/make core/cacerts
    core/rust core/musl core/zlib-musl core/bzip2-musl core/xz-musl core/libsodium-musl
    core/openssl-musl core/libarchive-musl
  )
fi
