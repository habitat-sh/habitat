pkg_name=libffi
pkg_version=3.2.1
pkg_origin=core
pkg_license=('mit')
pkg_maintainer="The Habitat Contributors <humans@habitat.sh>"
pkg_source=ftp://sourceware.org/pub/${pkg_name}/${pkg_name}-${pkg_version}.tar.gz
pkg_filename=${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=d06ebb8e1d9a22d19e38d63fdb83954253f39bedc5d46232a05645685722ca37
pkg_deps=(core/glibc core/libtool)
pkg_build_deps=(core/coreutils core/make core/gcc)
pkg_lib_dirs=(lib)
pkg_include_dirs=(include)

do_build() {
    ./configure --prefix=${pkg_prefix} --disable-multi-os-directory
    make
}
