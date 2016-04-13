pkg_name=psmisc
pkg_origin=chef
pkg_version=22.21
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('gpl')
pkg_source=http://downloads.sourceforge.net/psmisc/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=97323cad619210845b696d7d722c383852b2acb5c49b5b0852c4f29c77a8145a
pkg_deps=(chef/glibc chef/ncurses)
pkg_build_deps=(chef/coreutils chef/diffutils chef/patch chef/make chef/gcc)
pkg_bin_dirs=(bin)

do_prepare() {
  do_default_prepare

  # Thanks to: https://raw.githubusercontent.com/NixOS/nixpkgs/release-15.09/pkgs/os-specific/linux/psmisc/0001-Typo-in-fuser-makes-M-on-all-the-time.patch
  patch -p1 -i $PLAN_CONTEXT/0001-Typo-in-fuser-makes-M-on-all-the-time.patch
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
