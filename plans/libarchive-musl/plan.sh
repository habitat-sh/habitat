source ../libarchive/plan.sh

pkg_name=libarchive-musl
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_deps=(chef/musl chef/openssl-musl chef/zlib-musl chef/bzip2-musl chef/xz-musl)

do_prepare() {
  export CC=musl-gcc
  build_line "Setting CC=$CC"

  dynamic_linker="$(pkg_path_for musl)/lib/ld-musl-x86_64.so.1"
  LDFLAGS="$LDFLAGS -Wl,--dynamic-linker=$dynamic_linker"
}
