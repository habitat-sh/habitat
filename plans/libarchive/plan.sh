pkg_name=libarchive
pkg_distname=$pkg_name
pkg_origin=core
pkg_version=3.1.2
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('BSD')
pkg_source=http://www.libarchive.org/downloads/${pkg_distname}-${pkg_version}.tar.gz
pkg_shasum=eb87eacd8fe49e8d90c8fdc189813023ccc319c5e752b01fb6ad0cc7b2c53d5e
pkg_dirname=${pkg_distname}-${pkg_version}
pkg_deps=(core/glibc core/openssl core/zlib core/bzip2 core/xz)
pkg_build_deps=(core/gcc core/coreutils core/make)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)

do_build() {
    ./configure \
      --prefix=$pkg_prefix \
      --without-xml2 \
      --without-lzo2
  make
}
