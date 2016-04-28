pkg_name=man-pages
pkg_origin=core
pkg_version=4.02
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('gplv2')
pkg_source=http://ftp.kernel.org/pub/linux/docs/$pkg_name/${pkg_name}-${pkg_version}.tar.xz
pkg_shasum=48aacb75d522dd31978682c4fd8bc68e43c9a409bc4c7a126810e7610dff0dd3
pkg_build_deps=(core/coreutils core/make)

do_build() {
  return 0
}

do_install() {
  make install prefix=$pkg_prefix
}
