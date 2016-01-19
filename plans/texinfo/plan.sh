pkg_name=texinfo
pkg_derivation=chef
pkg_version=6.0
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_license=('gplv3+')
pkg_source=http://ftp.gnu.org/gnu/$pkg_name/${pkg_name}-${pkg_version}.tar.xz
pkg_shasum=11ba4358696e8be3b3c7cfc88b89cf69525791aeabf0ee0a59ca58ebbd3471e4
pkg_deps=(chef/glibc chef/ncurses chef/perl)
pkg_build_deps=(chef/gcc chef/coreutils chef/sed chef/bison chef/flex chef/grep chef/bash chef/gawk chef/libtool chef/diffutils chef/findutils chef/xz chef/gettext chef/gzip chef/make)
pkg_binary_path=(bin)
pkg_gpg_key=3853DA6B

do_check() {
  make check
}
