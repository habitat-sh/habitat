pkg_name=less
pkg_derivation=chef
pkg_version=481
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_license=('gplv3+')
pkg_source=http://www.greenwoodsoftware.com/$pkg_name/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=3fa38f2cf5e9e040bb44fffaa6c76a84506e379e47f5a04686ab78102090dda5
pkg_deps=(chef/glibc chef/ncurses chef/pcre)
pkg_build_deps=(chef/gcc chef/coreutils chef/sed chef/bison chef/flex chef/grep chef/bash chef/gawk chef/libtool)
pkg_binary_path=(bin)
pkg_gpg_key=3853DA6B

do_build() {
  ./configure \
    --prefix=$pkg_prefix \
    --sysconfdir=/etc \
    --with-regex=pcre
  make
}
