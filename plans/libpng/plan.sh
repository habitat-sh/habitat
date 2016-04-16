pkg_name=libpng
pkg_version=1.6.21
pkg_origin=core
pkg_license=('libpng')
pkg_source=http://download.sourceforge.net/${pkg_name}/${pkg_name}-${pkg_version}.tar.gz
pkg_filename=${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=b36a3c124622c8e1647f360424371394284f4c6c4b384593e478666c59ff42d3
pkg_deps=(core/glibc core/zlib)
pkg_build_deps=(core/gcc core/make core/coreutils core/diffutils
                core/autoconf core/automake)
pkg_lib_dirs=(lib)
pkg_include_dirs=(include)

do_build() {
  _zlib_dir=$(pkg_path_for zlib)

  export CPPFLAGS="${CPPFLAGS} ${CFLAGS}"
  ./configure --prefix=${pkg_prefix} \
              --host=x86_64-unknown-linux-gnu \
              --build=x86_64-unknown-linux-gnu \
              --disable-static \
              --with-zlib-prefix=${_zlib_dir} \ &&
  make && make install
}
