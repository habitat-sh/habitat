pkg_name=libarchive
pkg_origin=chef
pkg_version=3.1.2
pkg_license=('BSD')
pkg_source=http://www.libarchive.org/downloads/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=eb87eacd8fe49e8d90c8fdc189813023ccc319c5e752b01fb6ad0cc7b2c53d5e
pkg_gpg_key=3853DA6B
pkg_build_deps=(chef/gcc chef/coreutils chef/make)
pkg_deps=(chef/bzip2 chef/openssl chef/xz chef/zlib)
pkg_lib_dirs=(lib)
pkg_include_dirs=(include)

do_build() {
    ./configure \
      --prefix=$pkg_prefix \
      --without-xml2 \
      --without-lzo2
  make
}
