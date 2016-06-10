source ../wget/plan.sh

pkg_name=wget-static
pkg_maintainer="The Habitat Contributors <humans@habitat.sh>"
pkg_dirname=${pkg_distname}-${pkg_version}

# Throw the run deps into build deps as this will be static
pkg_build_deps=(core/linux-headers-musl core/musl "${pkg_build_deps[@]}" "${pkg_deps[@]}")
# Empty out the run deps array
pkg_deps=()

do_prepare() {
  CFLAGS="-I$(pkg_path_for linux-headers-musl)/include -I$(pkg_path_for musl)/include"
  build_line "Setting CFLAGS=$CFLAGS"

  LDFLAGS="-static $LDFLAGS"
  build_line "Setting LDFLAGS=$LDFLAGS"

  export CC=musl-gcc
  build_line "Setting CC=$CC"

  _wget_common_prepare
}
