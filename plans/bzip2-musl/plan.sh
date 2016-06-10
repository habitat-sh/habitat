source ../bzip2/plan.sh

pkg_name=bzip2-musl
pkg_maintainer="The Habitat Contributors <humans@habitat.sh>"
pkg_deps=(core/musl)

do_prepare() {
  _common_prepare

  export CC=musl-gcc
  build_line "Setting CC=$CC"

  dynamic_linker="$(pkg_path_for musl)/lib/ld-musl-x86_64.so.1"
  LDFLAGS="$LDFLAGS -Wl,--dynamic-linker=$dynamic_linker"
}
