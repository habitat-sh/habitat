pkg_name=libassuan
pkg_derivation=chef
pkg_version=2.4.2
pkg_license=('LGPL')
pkg_source=https://www.gnupg.org/ftp/gcrypt/${pkg_name}/${pkg_name}-${pkg_version}.tar.bz2
pkg_shasum=bb06dc81380b74bf1b64d5849be5c0409a336f3b4c45f20ac688e86d1b5bcb20
pkg_gpg_key=3853DA6B
pkg_binary_path=(bin)
pkg_lib_dirs=(lib)
pkg_include_dirs=(include)
pkg_deps=(chef/libgpg-error)

build() {
  ./configure \
    --prefix=$pkg_prefix \
    --with-libgpg-error-prefix=$(latest_package chef/libgpg-error)
  make
}

strip_binaries() {
  return 0
}
