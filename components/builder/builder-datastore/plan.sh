pkg_origin=core
pkg_name=builder-datastore
pkg_description="Datastore service for a Habitat Builder service"
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh"
pkg_license=("Apache-2.0")
pkg_deps=(core/postgresql)
pkg_build_deps=(core/git)
pkg_exports=(
  [port]=port
)
pkg_exposes=(port)
pkg_version() {
  git rev-list master --count
}

do_before() {
  update_pkg_version
}

do_build() {
  return 0
}

do_install() {
  return 0
}
