# shellcheck disable=2154
pkg_origin=core
pkg_name=hab-pkg-aci
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=("Apache-2.0")
pkg_deps=(core/coreutils
          core/findutils
          core/gawk
          core/grep
          core/bash
          core/acbuild
          core/hab-studio)
pkg_build_deps=()
pkg_bin_dirs=(bin)

program=$pkg_name

pkg_version() {
  if [[ -n "${DO_FAKE_RELEASE:-}" ]]; then
    cat "$SRC_PATH/../../VERSION_FAKE"
  else
    cat "$SRC_PATH/../../VERSION"
  fi
}
 
do_before() {
  do_default_before
  update_pkg_version
}

do_build() {
  cp -v "${SRC_PATH}/bin/${program}.sh" "${program}"


  # Use the bash from our dependency list as the shebang. Also, embed the
  # release version of the program.
  sed \
    -e "s,#!/bin/bash$,#!$(pkg_path_for bash)/bin/bash," \
    -e "s,@author@,$pkg_maintainer,g" \
    -e "s,@version@,$pkg_version/$pkg_release,g" \
    -i $program
}

do_install() {
  install -v -D "$program" "$pkg_prefix/bin/$program"
}
