pkg_name=python2
pkg_version=2.7.11
pkg_origin=core
pkg_license=('python')
pkg_dirname=Python-${pkg_version}
pkg_source=https://www.python.org/ftp/python/${pkg_version}/${pkg_dirname}.tgz
pkg_filename=${pkg_dirname}.tgz
pkg_shasum=82929b96fd6afc8da838b149107078c02fa1744b7e60999a8babbc0d3fa86fc6
pkg_deps=(core/glibc core/gcc-libs core/coreutils core/make core/ncurses core/zlib core/readline core/openssl core/bzip2)
pkg_build_deps=(core/linux-headers core/gcc)
pkg_lib_dirs=(lib)
pkg_bin_dirs=(bin)
pkg_include_dirs=(include Include)
pkg_interpreters=(bin/python bin/python2 bin/python2.7)

do_prepare() {
    sed -i.bak 's/#zlib/zlib/' Modules/Setup.dist
    sed -i -re "/(SSL=|_ssl|-DUSE_SSL|-lssl).*/ s|^#||" Modules/Setup.dist
}

do_build() {
    ./configure --prefix=${pkg_prefix} \
                --enable-shared \
                --with-ensurepip
    make
}
