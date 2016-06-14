source ../libsodium/plan.sh

pkg_name=libsodium-musl
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_deps=(core/musl)

do_prepare() {
  export CC=musl-gcc
  build_line "Setting CC=$CC"
}


# ----------------------------------------------------------------------------
# **NOTICE:** What follows are implementation details required for building a
# first-pass, "stage1" toolchain and environment. It is only used when running
# in a "stage1" Studio and can be safely ignored by almost everyone. Having
# said that, it performs a vital bootstrapping process and cannot be removed or
# significantly altered. Thank you!
# ----------------------------------------------------------------------------
if [[ "$STUDIO_TYPE" = "stage1" ]]; then
  pkg_build_deps=(core/gcc core/coreutils core/sed core/diffutils core/make)
fi
