pkg_name=boost159
pkg_origin=core
pkg_version=1.59.0
pkg_maintainer='The Habitat Maintainers <humans@habitat.sh>'
pkg_license=('Boost Software License')
pkg_source=http://downloads.sourceforge.net/project/boost/boost/1.59.0/boost_1_59_0.tar.gz
pkg_shasum=47f11c8844e579d02691a607fbd32540104a9ac7a2534a8ddaef50daf502baac
pkg_dirname=boost_1_59_0

pkg_deps=(
  core/glibc
  core/gcc-libs
)

pkg_build_deps=(
  core/glibc
  core/gcc-libs
  core/coreutils
  core/diffutils
  core/patch
  core/make
  core/gcc
  core/python2
  core/libxml2
  core/libxslt
  core/openssl
  core/which
)

pkg_lib_dirs=(lib)
pkg_include_dirs=(include)

do_build() {
  ./bootstrap.sh --prefix=$pkg_prefix
}

do_install() {
  ./b2 install --prefix=$pkg_prefix -q --debug-configuration -s NO_BZIP2=1
}
