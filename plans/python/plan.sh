pkg_name=python
pkg_version=3.5.1
pkg_origin=core
pkg_license=('python')
pkg_dirname=Python-${pkg_version}
pkg_source=https://www.python.org/ftp/python/${pkg_version}/${pkg_dirname}.tgz
pkg_filename=${pkg_dirname}.tgz
pkg_shasum=687e067d9f391da645423c7eda8205bae9d35edc0c76ef5218dcbe4cc770d0d7
pkg_deps=(core/glibc core/gcc-libs core/coreutils core/make core/ncurses core/zlib core/readline core/openssl core/bzip2)
pkg_build_deps=(core/linux-headers core/gcc)
pkg_lib_dirs=(lib)
pkg_bin_dirs=(bin)
pkg_include_dirs=(include Include)
pkg_interpreters=(bin/python bin/python3 bin/python3.5)

do_build() {
    ./configure --prefix=${pkg_prefix} \
                --enable-shared
    make
}
