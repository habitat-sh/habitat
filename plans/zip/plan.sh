pkg_name=zip
pkg_origin=core
pkg_version=3.0
_ver=${pkg_version/./}
pkg_license=('custom')
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_source=http://downloads.sourceforge.net/infozip/${pkg_name}${_ver}.tar.gz
pkg_shasum=f0e8bb1f9b7eb0b01285495a2699df3a4b766784c1765a8f1aeedf63c0806369
pkg_dirname="${pkg_name}${_ver}"
pkg_deps=(core/glibc)
pkg_build_deps=(core/coreutils core/make core/gcc)
pkg_bin_dirs=(bin)

do_build() {
  make -f unix/Makefile prefix=$pkg_prefix generic_gcc
}

do_install() {
  make -f unix/Makefile \
    prefix=$pkg_prefix \
    MANDIR=${pkg_prefix}/share/man/man1 \
    install
}
