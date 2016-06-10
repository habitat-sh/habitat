pkg_name=gettext
pkg_origin=core
pkg_version=0.19.6
pkg_maintainer="The Habitat Contributors <humans@habitat.sh>"
pkg_license=('gplv2+' 'lgpl2+')
pkg_source=http://ftp.gnu.org/gnu/$pkg_name/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=ed4b4c19bd3a3034eb6769500a3592ff616759ef43cf30586dbb7a17c9dd695d
pkg_deps=(core/glibc core/gcc-libs core/acl core/xz)
pkg_build_deps=(core/coreutils core/diffutils core/patch core/make core/gcc core/sed core/findutils)
pkg_bin_dirs=(bin)
pkg_lib_dirs=(lib)
pkg_include_dirs=(include)

do_build() {
  ./configure \
    --prefix=$pkg_prefix
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
  pkg_build_deps=(core/gcc core/coreutils core/sed core/diffutils core/findutils)
fi
