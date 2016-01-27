pkg_name=libtool
pkg_derivation=chef
pkg_version=2.4.6
pkg_license=('gplv2')
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_source=http://ftp.gnu.org/gnu/${pkg_name}/${pkg_name}-${pkg_version}.tar.gz
pkg_filename=${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=e3bd4d5d3d025a36c21dd6af7ea818a2afcd4dfc1ea5a17b39d7854bcd0c06e3
pkg_gpg_key=3853DA6B
pkg_deps=(chef/glibc)
pkg_lib_dirs=(lib)
pkg_include_dirs=(include)

do_prepare () {
  # Drop the dependency on `help2man` by skipping the generation of a man page
  sed \
    -e '/^dist_man1_MANS =/ s,^.*$,dist_man1_MANS = $(libtoolize_1),g' \
    -i Makefile.in
}
