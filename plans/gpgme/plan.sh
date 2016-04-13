pkg_name=gpgme
pkg_origin=chef
pkg_version=1.6.0
pkg_license=('LGPL')
pkg_source=https://www.gnupg.org/ftp/gcrypt/${pkg_name}/${pkg_name}-${pkg_version}.tar.bz2
pkg_shasum=b09de4197ac280b102080e09eaec6211d081efff1963bf7821cf8f4f9916099d
pkg_bin_dirs=(bin)
pkg_lib_dirs=(lib)
pkg_include_dirs=(include)
pkg_build_deps=(chef/gcc chef/coreutils chef/make)
pkg_deps=(chef/glibc chef/libassuan chef/libgpg-error)

do_build() {
  ./configure \
    --prefix=$pkg_prefix \
    --with-libgpg-error-prefix=$(pkg_path_for chef/libgpg-error) \
    --with-libassuan-prefix=$(pkg_path_for chef/libassuan)
  make
}
