pkg_name=mysql
pkg_origin=core
pkg_version=5.7.12
pkg_maintainer='The Habitat Maintainers <humans@habitat.sh>'
pkg_license=('GPLv2')
pkg_source=http://dev.mysql.com/get/Downloads/MySQL-5.7/mysql-5.7.12.tar.gz
pkg_shasum=32843cb6d22ab22cd2340262b53c0d6009b5bd41b1fa4102beda19635a5c1c87

pkg_deps=(
  core/glibc
  core/gcc-libs
  core/pcre
)

pkg_build_deps=(
  core/cmake
  core/coreutils
  core/diffutils
  core/patch
  core/make
  core/gcc
  core/bzip2-musl
  core/git
  core/curl
  core/which
  core/wget
  core/boost159
  core/python2
)

pkg_bin_dirs=(bin)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)

do_build() {
  cmake . -DLOCAL_BOOST_DIR=$(pkg_path_for boost159) \
          -DBOOST_INCLUDE_DIR=$(pkg_path_for boost159)/include \
          -DWITH_BOOST=$(pkg_path_for boost159) \
          -DCMAKE_INSTALL_PREFIX=$pkg_prefix
  make
}

do_install() {
  make install
}
