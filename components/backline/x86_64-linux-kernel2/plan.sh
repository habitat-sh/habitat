pkg_name=hab-backline
pkg_origin=core
pkg_version=$(cat "$SRC_PATH/../../VERSION")
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('Apache-2.0')
pkg_build_deps=()

pkg_deps=(core/hab-plan-build
          core/diffutils
          core/less
          core/make
          core/mg
          core/util-linux
          core/vim
          core/ncurses)

do_build() {
  return 0
}

do_install() {
  return 0
}
