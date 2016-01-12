pkg_name=attr
pkg_derivation=chef
pkg_version=2.4.47
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_license=('gplv2+')
pkg_source=http://download.savannah.gnu.org/releases/$pkg_name/$pkg_name-${pkg_version}.src.tar.gz
pkg_shasum=25772f653ac5b2e3ceeb89df50e4688891e21f723c460636548971652af0a859
pkg_build_deps=(chef/binutils chef/gcc)
pkg_deps=(chef/glibc)
pkg_binary_path=(bin)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)
pkg_gpg_key=3853DA6B

do_install() {
  make install install-dev install-lib
  chmod -v 755 $pkg_path/lib/libattr.so
}
