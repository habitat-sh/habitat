pkg_name=libedit
pkg_origin=chef
pkg_version=3.1.20150325
pkg_license=('bsd')
pkg_source=http://thrysoee.dk/editline/libedit-20150325-3.1.tar.gz
pkg_dirname=${pkg_name}-20150325-3.1
pkg_shasum=c88a5e4af83c5f40dda8455886ac98923a9c33125699742603a88a0253fcc8c5
pkg_gpg_key=3853DA6B
pkg_deps=(chef/glibc chef/ncurses)
pkg_build_deps=(chef/gcc chef/make chef/coreutils)
pkg_lib_dirs=(lib)
pkg_include_dirs=(include)

do_build() {
  ./configure --enable-widec --prefix=$pkg_prefix
}
