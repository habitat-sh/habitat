pkg_name=polipo
pkg_origin=mattray
pkg_version=1.1.1
pkg_maintainer="Matt Ray <matt@chef.io>"
pkg_license=('Apache-2')
pkg_source=https://github.com/jech/polipo/archive/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=de0afeeaa47df776420e59b69e9a78f68e8a7bc4a46b739772e1d91119ed2405
pkg_deps=(core/glibc)
pkg_build_deps=(core/gcc core/make)
pkg_bin_dirs=(bin)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)
pkg_expose=(8123)

pkg_dirname="${pkg_name}-${pkg_name}-${pkg_version}"

do_build() {
  make
}

do_install() {
  mkdir $pkg_prefix/bin/
  cp polipo $pkg_prefix/bin/
}
