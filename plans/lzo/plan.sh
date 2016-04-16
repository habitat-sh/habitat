pkg_name=lzo
pkg_origin=core
pkg_version=2.09
pkg_license=('GPL')
pkg_source=http://www.oberhumer.com/opensource/${pkg_name}/download/${pkg_name}-${pkg_version}.tar.gz
pkg_filename=${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=f294a7ced313063c057c504257f437c8335c41bfeed23531ee4e6a2b87bcb34c
pkg_deps=(core/glibc)
pkg_build_deps=(core/coreutils core/make core/gcc)
pkg_lib_dirs=(lib)
pkg_include_dirs=(include)

do_build() {
  ./configure \
    --prefix=$pkg_prefix \
    --enable-shared \
    --disable-static
  make
}
