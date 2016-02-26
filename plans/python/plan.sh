pkg_name=python
pkg_version=3.5.1
pkg_origin=chef
pkg_license=('python')
pkg_dirname=Python-${pkg_version}
pkg_source=https://www.python.org/ftp/python/${pkg_version}/${pkg_dirname}.tgz
pkg_filename=${pkg_dirname}.tgz
pkg_shasum=687e067d9f391da645423c7eda8205bae9d35edc0c76ef5218dcbe4cc770d0d7
pkg_gpg_key=3853DA6B
pkg_deps=(chef/glibc chef/gcc-libs chef/coreutils chef/make chef/ncurses chef/zlib chef/readline chef/openssl chef/bzip2)
pkg_build_deps=(chef/linux-headers chef/gcc)
pkg_lib_dirs=(lib)
pkg_binary_path=(bin)
pkg_include_dirs=(include Include)
pkg_interpreters=(bin/python bin/python3 bin/python3.5)

do_build() {
    ./configure --prefix=${pkg_prefix} \
                --enable-shared
    make
}
