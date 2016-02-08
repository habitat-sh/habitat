pkg_name=mpfr
pkg_derivation=chef
pkg_version=3.1.3
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_license=('lgpl')
pkg_source=http://www.mpfr.org/mpfr-current/${pkg_name}-${pkg_version}.tar.xz
pkg_shasum=6835a08bd992c8257641791e9a6a2b35b02336c8de26d0a8577953747e514a16
pkg_deps=(chef/glibc chef/gmp)
pkg_build_deps=(chef/coreutils chef/diffutils chef/patch chef/make chef/gcc chef/binutils)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)
pkg_gpg_key=3853DA6B

do_prepare() {
  do_default_prepare

  LDFLAGS="$LDFLAGS -Wl,-rpath=${LD_RUN_PATH},--enable-new-dtags"
  build_line "Updating LDFLAGS=$LDFLAGS"
}

do_build() {
  ./configure \
    --prefix=$pkg_prefix \
    --with-gmp=$(pkg_path_for chef/gmp) \
    --enable-thread-safe
  make -j$(nproc)
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
  pkg_build_deps=(chef/binutils)
fi
