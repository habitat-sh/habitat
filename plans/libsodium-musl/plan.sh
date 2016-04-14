source ../libsodium/plan.sh

pkg_name=libsodium-musl
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_deps=(chef/musl)

do_prepare() {
  export CC=musl-gcc
  build_line "Setting CC=$CC"
}
