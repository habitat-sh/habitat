pkg_name=git
pkg_version=2.7.4
pkg_origin=core
pkg_license=('gplv2')
pkg_source=https://www.kernel.org/pub/software/scm/git/${pkg_name}-${pkg_version}.tar.gz
pkg_filename=${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=7104c4f5d948a75b499a954524cb281fe30c6649d8abe20982936f75ec1f275b
pkg_deps=(core/glibc core/zlib core/perl core/curl core/gettext core/expat core/cacerts)
pkg_build_deps=(core/make core/gcc)
pkg_lib_dirs=(lib)
pkg_include_dirs=(include)
pkg_bin_dirs=(bin)

do_prepare() {
  _perl_path="$(pkg_path_for perl)/bin/perl"
  sed -e "s#/usr/bin/perl#${_perl_path}#g" -i Makefile
}

do_build() {
  ./configure --prefix=${pkg_prefix}
  make
}
