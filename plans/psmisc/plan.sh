pkg_name=psmisc
pkg_derivation=chef
pkg_version=22.21
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_license=('gpl')
pkg_source=http://downloads.sourceforge.net/psmisc/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=97323cad619210845b696d7d722c383852b2acb5c49b5b0852c4f29c77a8145a
pkg_deps=(chef/glibc chef/ncurses)
pkg_build_deps=(chef/binutils chef/gcc)
pkg_binary_path=(bin)
pkg_gpg_key=3853DA6B

do_prepare() {
  do_default_prepare

  # Thanks to: https://raw.githubusercontent.com/NixOS/nixpkgs/release-15.09/pkgs/os-specific/linux/psmisc/0001-Typo-in-fuser-makes-M-on-all-the-time.patch
  patch -p1 -i $PLAN_CONTEXT/0001-Typo-in-fuser-makes-M-on-all-the-time.patch
}
