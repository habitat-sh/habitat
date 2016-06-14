pkg_name=zlib
pkg_distname=$pkg_name
pkg_origin=core
pkg_version=1.2.8
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('zlib')
pkg_source=http://zlib.net/current/${pkg_distname}-${pkg_version}.tar.gz
pkg_shasum=36658cb768a54c1d4dec43c3116c27ed893e88b02ecfcb44f2166f9c0b7f2a0d
pkg_dirname=${pkg_distname}-${pkg_version}
pkg_deps=(core/glibc)
pkg_build_deps=(core/coreutils core/diffutils core/patch core/make core/gcc)
pkg_lib_dirs=(lib)
pkg_include_dirs=(include)

do_prepare() {
  do_default_prepare

  # Add explicit linker instructions as the binutils we are using may have its
  # own dynamic linker defaults. This is necessary because this Plan is built
  # before the `binutils` Plan which will set the new `glibc` dynamic
  # linker for all later Plans.
  dynamic_linker="$(pkg_path_for glibc)/lib/ld-linux-x86-64.so.2"
  LDFLAGS="$LDFLAGS -Wl,-rpath=${LD_RUN_PATH},--enable-new-dtags"
  LDFLAGS="$LDFLAGS -Wl,--dynamic-linker=$dynamic_linker"
  export LDFLAGS
  build_line "Updating LDFLAGS=$LDFLAGS"
}


# ----------------------------------------------------------------------------
# **NOTICE:** What follows are implementation details required for building a
# first-pass, "stage1" toolchain and environment. It is only used when running
# in a "stage1" Studio and can be safely ignored by almost everyone. Having
# said that, it performs a vital bootstrapping process and cannot be removed or
# significantly altered. Thank you!
# ----------------------------------------------------------------------------
if [[ "$STUDIO_TYPE" = "stage1" ]]; then
  pkg_build_deps=()
fi
