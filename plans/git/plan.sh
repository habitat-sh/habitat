pkg_name=git
pkg_version=2.7.4
pkg_origin=chef
pkg_license=('gplv2')
pkg_source=https://www.kernel.org/pub/software/scm/git/${pkg_name}-${pkg_version}.tar.gz
pkg_filename=${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=7104c4f5d948a75b499a954524cb281fe30c6649d8abe20982936f75ec1f275b
pkg_deps=(chef/glibc chef/zlib chef/perl chef/curl chef/gettext chef/expat chef/cacerts)
pkg_build_deps=(chef/make chef/gcc)
pkg_lib_dirs=(lib)
pkg_include_dirs=(include)
pkg_bin_dirs=(bin)

do_prepare() {
  _perl_path="$(pkg_path_for chef/perl)/bin/perl"
  sed -e "s#/usr/bin/perl#${_perl_path}#g" -i Makefile
}

do_build() {
  ./configure --prefix=${pkg_prefix}
  make
}
