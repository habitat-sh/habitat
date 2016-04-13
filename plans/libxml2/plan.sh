pkg_name=libxml2
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_origin=chef
pkg_version=2.9.2
pkg_license=('MIT')
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_source=http://xmlsoft.org/sources/${pkg_name}-${pkg_version}.tar.gz
pkg_deps=(chef/zlib chef/glibc)
pkg_build_deps=(chef/coreutils chef/make chef/gcc chef/m4)
pkg_filename=${pkg_name}-${pkg_version}.tar.xz
pkg_lib_dirs=(lib)
pkg_include_dirs=(include)
pkg_bin_dirs=(bin)
pkg_shasum=5178c30b151d044aefb1b08bf54c3003a0ac55c59c866763997529d60770d5bc

do_build() {
  ./configure --prefix=${pkg_prefix} --without-python  --with-zlib=$(pkg_path_for chef/zlib)
  make
}
