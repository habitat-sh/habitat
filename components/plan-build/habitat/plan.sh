# shellcheck disable=2034
pkg_name=hab-plan-build
pkg_origin=core
pkg_version=$(cat "$SRC_PATH/../../VERSION")
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('Apache-2.0')
pkg_bin_dirs=(bin)

pkg_deps=(core/bash/4.3.42/20170513213519
          core/binutils/2.25.1/20170513201927
          core/bzip2/1.0.6/20170513212938
          core/coreutils/8.25/20170513213226
          core/file/5.24/20170513201915
          core/findutils/4.4.2/20170513214305
          core/gawk/4.1.3/20170513213646
          core/grep/2.22/20170513213444
          core/gzip/1.6/20170513214605
          core/hab
          core/rq/0.9.2/20170612005822
          core/sed/4.2.2/20170513213123
          core/tar/1.29/20170513213607
          core/unzip/6.0/20180310001951
          core/wget/1.19.1/20171024102323
          core/xz/5.2.2/20170513214327)
pkg_build_deps=(core/bats/0.4.0/20170514013659)

program=$pkg_name

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
  install -D "$SRC_PATH"/bin/composite_build_functions.bash "$pkg_prefix"/bin/
  install -D "$SRC_PATH"/bin/environment.bash "$pkg_prefix"/bin/
}
