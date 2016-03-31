pkg_name=python2
pkg_version=2.7.11
pkg_origin=chef
pkg_license=('python')
pkg_dirname=Python-${pkg_version}
pkg_source=https://www.python.org/ftp/python/${pkg_version}/${pkg_dirname}.tgz
pkg_filename=${pkg_dirname}.tgz
pkg_shasum=82929b96fd6afc8da838b149107078c02fa1744b7e60999a8babbc0d3fa86fc6
pkg_gpg_key=3853DA6B
pkg_deps=(chef/glibc chef/gcc-libs chef/coreutils chef/make chef/ncurses chef/zlib chef/readline chef/openssl chef/bzip2)
pkg_build_deps=(chef/linux-headers chef/gcc)
pkg_lib_dirs=(lib)
pkg_bin_dirs=(bin)
pkg_include_dirs=(include Include)
pkg_interpreters=(bin/python bin/python2 bin/python2.7)

do_build() {
    ./configure --prefix=${pkg_prefix} \
                --enable-shared
    make
}
