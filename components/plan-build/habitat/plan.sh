# shellcheck disable=2034
pkg_name=hab-plan-build
pkg_origin=core
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('Apache-2.0')
pkg_bin_dirs=(bin)

pkg_deps=(core/bash
          core/binutils
          core/coreutils
          core/file
          core/findutils
          core/gawk
          core/grep
          core/gzip
          core/hab
          core/rq
          core/sed
          core/tar
          core/unzip
          core/wget
          core/xz)
pkg_build_deps=(core/bats)

program=$pkg_name

pkg_version() {
  cat "$SRC_PATH/../../VERSION"
}

do_before() {
  do_default_before
  update_pkg_version
}

do_build() {
  cp -v "$SRC_PATH"/bin/${program}.sh "$CACHE_PATH/$program"

  # Use the bash from our dependency list as the shebang. Also, embed the
  # release version of the program.
  # shellcheck disable=2154
  sed \
    -e "s,#!/bin/bash\$,#!$(pkg_path_for bash)/bin/bash," \
    -e "s,^HAB_PLAN_BUILD=0\.0\.1\$,HAB_PLAN_BUILD=$pkg_version/$pkg_release," \
    -e "s,^pkg_target='@@pkg_target@@'\$,pkg_target='$pkg_target'," \
    -i "$CACHE_PATH/$program"
}

do_check() {
  bats test
}

do_install() {
  # shellcheck disable=2154
  install -D "$CACHE_PATH/$program" "$pkg_prefix"/bin/$program
  install -D "$SRC_PATH"/bin/shared.bash "$pkg_prefix"/bin/
  install -D "$SRC_PATH"/bin/public.bash "$pkg_prefix"/bin/
  install -D "$SRC_PATH"/bin/environment.bash "$pkg_prefix"/bin/
}
