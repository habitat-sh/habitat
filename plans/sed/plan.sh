pkg_name=sed
pkg_origin=chef
pkg_version=4.2.2
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('gplv3')
pkg_source=http://ftp.gnu.org/gnu/$pkg_name/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=fea0a94d4b605894f3e2d5572e3f96e4413bcad3a085aae7367c2cf07908b2ff
pkg_deps=(chef/glibc chef/acl)
pkg_build_deps=(chef/coreutils chef/diffutils chef/patch chef/make chef/gcc)
pkg_bin_dirs=(bin)
pkg_gpg_key=3853DA6B

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
  pkg_build_deps=(chef/gcc)
fi
