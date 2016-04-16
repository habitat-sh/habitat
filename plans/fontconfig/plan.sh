pkg_name=fontconfig
pkg_version=2.11.94
pkg_origin=core
pkg_license=('fontconfig')
pkg_source=https://www.freedesktop.org/software/fontconfig/release/${pkg_name}-${pkg_version}.tar.bz2
pkg_filename=${pkg_name}-${pkg_version}.tar.bz2
pkg_shasum=d763c024df434146f3352448bc1f4554f390c8a48340cef7aa9cc44716a159df
pkg_deps=(core/glibc)
pkg_build_deps=(core/gcc core/make core/coreutils core/python
                core/pkg-config core/freetype core/expat
                core/diffutils core/libtool core/m4 core/automake
                core/autoconf core/file)
pkg_lib_dirs=(lib)
pkg_include_dirs=(include)

do_prepare() {
  # Borrowing this pro tip from ArchLinux!
  # https://projects.archlinux.org/svntogit/packages.git/tree/trunk/PKGBUILD?h=packages/fontconfig#n34
  # this seems to run libtoolize though...
  autoreconf -fi

  _file_path="$(pkg_path_for file)/bin/file"
  _uname_path="$(pkg_path_for coreutils)/bin/uname"

  sed -e "s#/usr/bin/file#${_file_path}#g" -i configure
  sed -e "s#/usr/bin/uname#${_uname_path}#g" -i configure
}

do_build() {
  export PKG_CONFIG_PATH="$(pkg_path_for freetype)/lib/pkgconfig:$(pkg_path_for expat)/lib/pkgconfig"

  ./configure --sysconfdir=${pkg_prefix}/etc \
              --prefix=${pkg_prefix} \
              --disable-static \
              --mandir=${pkg_prefix}/man
  make
  make install
}
