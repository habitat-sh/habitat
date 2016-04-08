pkg_name=gmp
pkg_origin=chef
pkg_version=6.1.0
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('gplv3')
pkg_source=http://ftp.gnu.org/gnu/$pkg_name/${pkg_name}-${pkg_version}.tar.xz
pkg_shasum=68dadacce515b0f8a54f510edf07c1b636492bcdb8e8d54c56eb216225d16989
pkg_build_deps=(chef/coreutils chef/diffutils chef/patch chef/make chef/gcc chef/binutils chef/m4)
pkg_deps=(chef/glibc)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)
pkg_gpg_key=3853DA6B

do_prepare() {
  do_default_prepare

  # Set RUNPATH for c++ compiled code
  LDFLAGS="$LDFLAGS -Wl,-rpath=${LD_RUN_PATH},--enable-new-dtags"
  build_line "Updating LDFLAGS=$LDFLAGS"
}

do_build() {
  ./configure \
    --prefix=$pkg_prefix \
    --build=x86_64-unknown-linux-gnu
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
  pkg_build_deps=(chef/binutils chef/m4)
fi
