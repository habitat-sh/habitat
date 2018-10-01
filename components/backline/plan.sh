pkg_name=hab-backline
pkg_origin=core
pkg_version=$(cat "$SRC_PATH/../../VERSION")
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('Apache-2.0')
pkg_build_deps=()

pkg_deps=(core/hab-plan-build
          core/diffutils/3.3/20170513214227
          core/less/481/20170513213936
          core/make/4.2.1/20170513214620
          core/mg/20160118/20170513220846
          core/util-linux/2.27.1/20170513214732
          core/vim/8.0.0004/20170513220742
          core/ncurses/6.0/20170513213009)

do_build() {
  return 0
}

do_install() {
  return 0
}
