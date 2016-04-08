pkg_name=patch
pkg_origin=chef
pkg_version=2.7.5
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('gplv3+')
pkg_source=http://ftp.gnu.org/gnu/$pkg_name/${pkg_name}-${pkg_version}.tar.xz
pkg_shasum=fd95153655d6b95567e623843a0e77b81612d502ecf78a489a4aed7867caa299
pkg_deps=(chef/glibc chef/attr)
pkg_build_deps=(chef/coreutils chef/diffutils chef/make chef/gcc chef/sed)
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
  pkg_build_deps=(chef/gcc chef/coreutils chef/sed chef/diffutils chef/make)
fi
