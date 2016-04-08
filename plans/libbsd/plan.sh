pkg_name=libbsd
pkg_origin=chef
pkg_version=0.8.1
pkg_license=('custom')
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_source=http://libbsd.freedesktop.org/releases/${pkg_name}-${pkg_version}.tar.xz
pkg_shasum=adbc8781ad720bce939b689f38a9f0247732a36792147a7c28027c393c2af9b0
pkg_deps=(chef/glibc)
pkg_build_deps=(chef/coreutils chef/diffutils chef/patch chef/make chef/gcc chef/sed)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)
pkg_gpg_key=3853DA6B

do_install() {
  do_default_install

  # Install license file from README
  install -Dm644 COPYING "$pkg_prefix/share/licenses/LICENSE"
}


# ----------------------------------------------------------------------------
# **NOTICE:** What follows are implementation details required for building a
# first-pass, "stage1" toolchain and environment. It is only used when running
# in a "stage1" Studio and can be safely ignored by almost everyone. Having
# said that, it performs a vital bootstrapping process and cannot be removed or
# significantly altered. Thank you!
# ----------------------------------------------------------------------------
if [[ "$STUDIO_TYPE" = "stage1" ]]; then
  pkg_build_deps=(chef/gcc chef/coreutils chef/sed chef/diffutils chef/make chef/patch)
fi
