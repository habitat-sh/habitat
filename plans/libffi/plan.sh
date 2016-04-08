pkg_name=libffi
pkg_version=3.2.1
pkg_origin=chef
pkg_license=('mit')
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_source=ftp://sourceware.org/pub/${pkg_name}/${pkg_name}-${pkg_version}.tar.gz
pkg_filename=${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=d06ebb8e1d9a22d19e38d63fdb83954253f39bedc5d46232a05645685722ca37
pkg_gpg_key=3853DA6B
pkg_deps=(chef/glibc chef/libtool)
pkg_build_deps=(chef/coreutils chef/make chef/gcc)
pkg_lib_dirs=(lib)
pkg_include_dirs=(include)

do_build() {
    ./configure --prefix=${pkg_prefix} --disable-multi-os-directory
    make
}
