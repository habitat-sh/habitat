pkg_name=hab-plan-build-test
pkg_origin=core
pkg_version=$(cat "$PLAN_CONTEXT/../../VERSION")
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=("Apache-2.0")
pkg_deps=(
  core/hab-plan-build
  core/bash/4.3.42/20170513213519
  core/bats/0.4.0/20170514013659
)
# pkg_build_deps=(core/make core/gcc)
pkg_bin_dirs=(bin)
pkg_description="Test wrapper for Habitat plans"
pkg_upstream_url="https://www.habitat.sh"

do_build() {
  cp -v "$PLAN_CONTEXT"/bin/${pkg_name}.sh "$CACHE_PATH/$pkg_name"

  # Use the bash from our dependency list as the shebang. Also, embed the
  # release version of the program.
  # shellcheck disable=2154
  sed \
    -e "s,^HAB_PLAN_BUILD=0\.0\.1\$,HAB_PLAN_BUILD=$pkg_version/$pkg_release," \
    -e "s,#!/bin/bash\$,#!$(pkg_path_for bash)/bin/bash," \
    -e "s,^pkg_target='@@pkg_target@@'\$,pkg_target='$pkg_target'," \
    -i "$CACHE_PATH/$pkg_name"
}

do_install() {
  # shellcheck disable=2154
  install -D "$CACHE_PATH/$pkg_name" "$pkg_prefix"/bin/$pkg_name
}
