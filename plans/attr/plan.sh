pkg_name=attr
pkg_origin=core
pkg_version=2.4.47
pkg_maintainer="The Habitat Contributors <humans@habitat.sh>"
pkg_license=('gplv2+')
pkg_source=http://download.savannah.gnu.org/releases/$pkg_name/$pkg_name-${pkg_version}.src.tar.gz
pkg_shasum=25772f653ac5b2e3ceeb89df50e4688891e21f723c460636548971652af0a859
pkg_deps=(core/glibc)
pkg_build_deps=(core/coreutils core/diffutils core/patch core/make core/gcc core/gettext)
pkg_bin_dirs=(bin)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)

do_install() {
  make install install-dev install-lib
  chmod -v 755 $pkg_prefix/lib/libattr.so
}


# ----------------------------------------------------------------------------
# **NOTICE:** What follows are implementation details required for building a
# first-pass, "stage1" toolchain and environment. It is only used when running
# in a "stage1" Studio and can be safely ignored by almost everyone. Having
# said that, it performs a vital bootstrapping process and cannot be removed or
# significantly altered. Thank you!
# ----------------------------------------------------------------------------
if [[ "$STUDIO_TYPE" = "stage1" ]]; then
  pkg_build_deps=(core/gcc)
fi
