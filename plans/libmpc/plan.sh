pkg_name=libmpc
pkg_distname=mpc
pkg_origin=core
pkg_version=1.0.3
pkg_maintainer="The Habitat Contributors <humans@habitat.sh>"
pkg_license=('lgpl')
pkg_source=http://www.multiprecision.org/mpc/download/${pkg_distname}-${pkg_version}.tar.gz
pkg_shasum=617decc6ea09889fb08ede330917a00b16809b8db88c29c31bfbb49cbf88ecc3
pkg_deps=(core/glibc core/gmp core/mpfr)
pkg_build_deps=(core/coreutils core/diffutils core/patch core/make core/gcc core/binutils)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)
pkg_dirname=${pkg_distname}-${pkg_version}

do_prepare() {
  do_default_prepare

  LDFLAGS="$LDFLAGS -Wl,-rpath=${LD_RUN_PATH},--enable-new-dtags"
  build_line "Updating LDFLAGS=$LDFLAGS"
}

do_check() {
  make check
}


# ----------------------------------------------------------------------------
# **NOTICE:** What follows are implementation details required for building a
# first-pass, "stage1" toolchain and environment. It is only used when running
# in a "stage1" Studio and can be safely ignored by almost everyone. Having
# said that, it performs a vital bootstrapping process and cannot be removed or
# significantly altered. Thank you!
# ----------------------------------------------------------------------------
if [[ "$STUDIO_TYPE" = "stage1" ]]; then
  pkg_build_deps=(core/binutils)
fi
