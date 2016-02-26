pkg_name=automake
pkg_origin=chef
pkg_version=1.15
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_license=('gplv2+')
pkg_source=http://ftp.gnu.org/gnu/$pkg_name/${pkg_name}-${pkg_version}.tar.xz
pkg_shasum=9908c75aabd49d13661d6dcb1bc382252d22cc77bf733a2d55e87f2aa2db8636
pkg_deps=(chef/perl)
pkg_build_deps=(chef/coreutils chef/diffutils chef/patch chef/make chef/gcc chef/autoconf)
pkg_binary_path=(bin)
pkg_gpg_key=3853DA6B


# ----------------------------------------------------------------------------
# **NOTICE:** What follows are implementation details required for building a
# first-pass, "stage1" toolchain and environment. It is only used when running
# in a "stage1" Studio and can be safely ignored by almost everyone. Having
# said that, it performs a vital bootstrapping process and cannot be removed or
# significantly altered. Thank you!
# ----------------------------------------------------------------------------
if [[ "$STUDIO_TYPE" = "stage1" ]]; then
  pkg_build_deps=(chef/gcc chef/coreutils chef/diffutils chef/autoconf)
fi
