pkg_name=acl
pkg_derivation=chef
pkg_version=2.2.52
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_license=('lgpl')
pkg_source=http://download.savannah.gnu.org/releases/$pkg_name/$pkg_name-${pkg_version}.src.tar.gz
pkg_shasum=179074bb0580c06c4b4137be4c5a92a701583277967acdb5546043c7874e0d23
pkg_build_deps=(chef/binutils chef/gcc)
pkg_deps=(chef/glibc chef/attr)
pkg_binary_path=(bin)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)
pkg_gpg_key=3853DA6B

do_prepare() {
  # Fix a bug that causes `getfacl -e` to segfault on overly long group name.
  #
  # Thanks to: http://www.linuxfromscratch.org/lfs/view/stable/chapter06/acl.html
  sed -i -e "/TABS-1;/a if (x > (TABS-1)) x = (TABS-1);" \
    libacl/__acl_to_any_text.c
}

do_install() {
  make install install-dev install-lib
  chmod -v 755 $pkg_path/lib/libacl.so
}
