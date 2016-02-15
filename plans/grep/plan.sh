pkg_name=grep
pkg_derivation=chef
pkg_version=2.22
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_license=('gplv3+')
pkg_source=http://ftp.gnu.org/gnu/$pkg_name/${pkg_name}-${pkg_version}.tar.xz
pkg_shasum=ca91d22f017bfcb503d4bc3b44295491c89a33a3df0c3d8b8614f2d3831836eb
pkg_deps=(chef/glibc chef/pcre)
pkg_build_deps=(chef/coreutils chef/diffutils chef/patch chef/make chef/gcc chef/perl)
pkg_binary_path=(bin)
pkg_gpg_key=3853DA6B

do_prepare() {
  patch -p1 -i $PLAN_CONTEXT/disable-perf-related-test.patch
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
  pkg_build_deps=(chef/gcc chef/coreutils)
fi
