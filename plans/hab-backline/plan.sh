pkg_name=hab-backline
pkg_origin=core
pkg_version=0.6.0
pkg_maintainer="The Habitat Contributors <humans@habitat.sh>"
pkg_license=('apachev2')
pkg_source=nosuchfile.tar.gz
pkg_build_deps=()

pkg_deps=(
  core/hab-plan-build
  core/diffutils
  core/less
  core/make
  core/mg
  core/util-linux
  core/vim
)

do_download() {
  return 0
}

do_verify() {
  return 0
}

do_unpack() {
  return 0
}

do_prepare() {
  return 0
}

do_build() {
  return 0
}

do_install() {
  return 0
}
