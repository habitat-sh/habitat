pkg_name=make
pkg_derivation=chef
pkg_version=4.1
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_license=('gplv3+')
pkg_source=http://ftp.gnu.org/gnu/$pkg_name/${pkg_name}-${pkg_version}.tar.bz2
pkg_shasum=0bc7613389650ee6a24554b52572a272f7356164fd2c4132b0bcf13123e4fca5
pkg_deps=(chef/glibc)
pkg_build_deps=(chef/gcc chef/coreutils chef/sed chef/bison chef/flex chef/grep chef/bash chef/gawk chef/libtool chef/diffutils chef/findutils chef/xz chef/gettext chef/gzip chef/perl)
pkg_binary_path=(bin)
pkg_include_dirs=(include)
pkg_gpg_key=3853DA6B

do_prepare() {
  do_default_prepare

  # Don't look for library dependencies in the root system (i.e. `/lib`,
  # `/usr/lib`, etc.)
  patch -p1 -i $PLAN_CONTEXT/no-sys-dirs.patch
}

do_check() {
  make check
}
