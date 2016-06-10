pkg_name=less
pkg_origin=core
pkg_version=481
pkg_maintainer="The Habitat Contributors <humans@habitat.sh>"
pkg_license=('gplv3+')
pkg_source=http://www.greenwoodsoftware.com/$pkg_name/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=3fa38f2cf5e9e040bb44fffaa6c76a84506e379e47f5a04686ab78102090dda5
pkg_deps=(core/glibc core/ncurses core/pcre)
pkg_build_deps=(core/coreutils core/diffutils core/patch core/make core/gcc)
pkg_bin_dirs=(bin)

do_build() {
  ./configure \
    --prefix=$pkg_prefix \
    --sysconfdir=/etc \
    --with-regex=pcre
  make
}


# ----------------------------------------------------------------------------
# **NOTICE:** What follows are implementation details required for building a
# first-pass, "stage1" toolchain and environment. It is only used when running
# in a "stage1" Studio and can be safely ignored by almost everyone. Having
# said that, it performs a vital bootstrapping process and cannot be removed or
# significantly altered. Thank you!
# ----------------------------------------------------------------------------
if [[ "$STUDIO_TYPE" = "stage1" ]]; then
  pkg_build_deps=(core/gcc core/coreutils)
fi
