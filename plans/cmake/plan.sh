pkg_name=cmake
pkg_origin=core
pkg_version=3.5.2
pkg_maintainer='The Habitat Maintainers <humans@habitat.sh>'
pkg_license=('custom')
pkg_source=https://cmake.org/files/v3.5/cmake-3.5.2.tar.gz
pkg_shasum=92d8410d3d981bb881dfff2aed466da55a58d34c7390d50449aa59b32bb5e62a

pkg_deps=(
  core/glibc
  core/gcc-libs
)

pkg_build_deps=(
  core/coreutils
  core/diffutils
  core/make
  core/gcc
  core/curl
  core/zlib
  core/bzip2
)

pkg_lib_dirs=(lib)
pkg_include_dirs=(include)
pkg_bin_dirs=(bin)

do_build() {
  ./bootstrap
  ./configure --prefix=$pkg_prefix
  make
}
