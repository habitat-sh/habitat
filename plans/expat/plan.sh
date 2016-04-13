pkg_name=expat
pkg_origin=chef
pkg_version=2.1.0
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('mit')
pkg_source=http://downloads.sourceforge.net/sourceforge/expat/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=823705472f816df21c8f6aa026dd162b280806838bb55b3432b0fb1fcca7eb86
pkg_deps=(chef/glibc chef/gcc-libs)
pkg_build_deps=(chef/coreutils chef/diffutils chef/patch chef/make chef/gcc)
pkg_bin_dirs=(bin)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)

do_prepare() {
  do_default_prepare

  # Patch for CVE-2015-1283
  #
  # Thanks to: https://github.com/NixOS/nixpkgs/blob/release-15.09/pkgs/development/libraries/expat/default.nix
  patch -p1 -i $PLAN_CONTEXT/CVE-2015-1283.patch
}

do_check() {
  # Set `LDFLAGS` for the c++ test code to find libstdc++
  make check LDFLAGS="$LDFLAGS -lstdc++"
}

do_install() {
  do_default_install

  # Install license file
  install -Dm644 COPYING "$pkgdir/share/licenses/COPYING"
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
