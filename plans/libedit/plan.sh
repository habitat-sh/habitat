pkg_name=libedit
pkg_origin=core
pkg_version=3.1.20150325
pkg_license=('bsd')
pkg_source=http://thrysoee.dk/editline/libedit-20150325-3.1.tar.gz
pkg_dirname=${pkg_name}-20150325-3.1
pkg_shasum=c88a5e4af83c5f40dda8455886ac98923a9c33125699742603a88a0253fcc8c5
pkg_deps=(core/glibc core/ncurses)
pkg_build_deps=(core/gcc core/make core/coreutils)
pkg_lib_dirs=(lib)
pkg_include_dirs=(include)

do_build() {
  ./configure --enable-widec --prefix=$pkg_prefix
}
