source ../zlib/plan.sh

pkg_name=zlib-musl
pkg_maintainer="The Habitat Contributors <humans@habitat.sh>"
pkg_deps=(core/musl)

do_prepare() {
  export CC=musl-gcc
  build_line "Setting CC=$CC"
}
