source ../openssl/plan.sh

pkg_name=openssl-musl
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_deps=(core/musl core/zlib-musl core/cacerts)

do_prepare() {
  PLAN_CONTEXT=$(abspath $PLAN_CONTEXT/../openssl) _common_prepare

  dynamic_linker="$(pkg_path_for musl)/lib/ld-musl-x86_64.so.1"
  LDFLAGS="$LDFLAGS -Wl,--dynamic-linker=$dynamic_linker"

  export BUILD_CC=musl-gcc
  build_line "Setting BUILD_CC=$BUILD_CC"
}
