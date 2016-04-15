pkg_name=libltdl
pkg_origin=core
pkg_version=2.4.6
pkg_license=('GPL')
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_source=https://ftp.gnu.org/pub/gnu/libtool/libtool-${pkg_version}.tar.xz
pkg_deps=(core/glibc)
pkg_filename=${pkg_name}-${pkg_version}.tar.xz
pkg_dirname=libtool-${pkg_version}
pkg_lib_dirs=(lib)
pkg_include_dirs=(include)
pkg_shasum=7c87a8c2c8c0fc9cd5019e402bed4292462d00a718a7cd5f11218153bf28b26f

do_prepare() {
  # Drop the dependency on `help2man` by skipping the generation of a man page
  sed \
    -e '/^dist_man1_MANS =/ s,^.*$,dist_man1_MANS = $(libtoolize_1),g' \
    -i Makefile.in
}

do_build() {
  ./configure
  make
}

do_install() {
  make install prefix=${pkg_prefix}
}
