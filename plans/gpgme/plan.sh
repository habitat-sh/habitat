pkg_name=gpgme
pkg_origin=core
pkg_version=1.6.0
pkg_license=('LGPL')
pkg_source=https://www.gnupg.org/ftp/gcrypt/${pkg_name}/${pkg_name}-${pkg_version}.tar.bz2
pkg_shasum=b09de4197ac280b102080e09eaec6211d081efff1963bf7821cf8f4f9916099d
pkg_bin_dirs=(bin)
pkg_lib_dirs=(lib)
pkg_include_dirs=(include)
pkg_build_deps=(core/gcc core/coreutils core/make)
pkg_deps=(core/glibc core/libassuan core/libgpg-error)

do_build() {
  ./configure \
    --prefix=$pkg_prefix \
    --with-libgpg-error-prefix=$(pkg_path_for libgpg-error) \
    --with-libassuan-prefix=$(pkg_path_for libassuan)
  make
}
