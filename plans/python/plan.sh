pkg_name=python
pkg_version=3.5.2
pkg_origin=core
pkg_license=('python')
pkg_dirname=Python-${pkg_version}
pkg_source=https://www.python.org/ftp/python/${pkg_version}/${pkg_dirname}.tgz
pkg_filename=${pkg_dirname}.tgz
pkg_shasum=1524b840e42cf3b909e8f8df67c1724012c7dc7f9d076d4feef2d3eff031e8a0
pkg_deps=(core/glibc core/gcc-libs core/coreutils core/make core/ncurses core/zlib core/readline core/openssl core/bzip2)
pkg_build_deps=(core/linux-headers core/gcc)
pkg_lib_dirs=(lib)
pkg_bin_dirs=(bin)
pkg_include_dirs=(include Include)
pkg_interpreters=(bin/python bin/python3 bin/python3.5)

do_prepare() {
    sed -i.bak 's/#zlib/zlib/' Modules/Setup.dist
    sed -i -re "/(SSL=|_ssl|-DUSE_SSL|-lssl).*/ s|^#||" Modules/Setup.dist
}

do_build() {
    ./configure --prefix=${pkg_prefix} \
                --enable-shared
    make
}
