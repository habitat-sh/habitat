pkg_name=autoconf
pkg_origin=core
pkg_version=2.69
pkg_maintainer="The Habitat Contributors <humans@habitat.sh>"
pkg_license=('gplv2+')
pkg_source=http://ftp.gnu.org/gnu/$pkg_name/${pkg_name}-${pkg_version}.tar.xz
pkg_shasum=64ebcec9f8ac5b2487125a86a7760d2591ac9e1d3dbd59489633f9de62a57684
pkg_deps=(core/m4 core/perl)
pkg_build_deps=(core/coreutils core/diffutils core/patch core/make core/gcc core/sed core/gawk)
pkg_bin_dirs=(bin)


# ----------------------------------------------------------------------------
# **NOTICE:** What follows are implementation details required for building a
# first-pass, "stage1" toolchain and environment. It is only used when running
# in a "stage1" Studio and can be safely ignored by almost everyone. Having
# said that, it performs a vital bootstrapping process and cannot be removed or
# significantly altered. Thank you!
# ----------------------------------------------------------------------------
if [[ "$STUDIO_TYPE" = "stage1" ]]; then
  pkg_build_deps=(core/gcc core/coreutils core/sed core/gawk core/diffutils)
fi
