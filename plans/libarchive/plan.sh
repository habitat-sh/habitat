pkg_name=libarchive
pkg_derivation=chef
pkg_version=3.1.2
pkg_license=('BSD')
pkg_source=https://github.com/${pkg_name}/${pkg_name}/archive/v${pkg_version}.tar.gz
pkg_filename=${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=9543ed21ac18fecc238dec4a6fae6abf1cc449dbad5940ca0d80202e0a7237a4
pkg_gpg_key=3853DA6B
pkg_deps=(chef/bzip2 chef/openssl chef/xz chef/zlib)
pkg_lib_dirs=(lib)
pkg_include_dirs=(include)

do_build() {
  build/autogen.sh
  ./configure \
    --prefix=$pkg_prefix \
    --without-xml2 \
    --without-lzo2
  make
}
