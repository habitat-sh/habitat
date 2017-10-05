pkg_name=hab-pkg-cfize
pkg_origin=core
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('Apache-2.0')
pkg_description="Habitat Cloud Foundry image exporter"
pkg_upstream_url="https://github.com/habitat-sh/habitat"
pkg_deps=(
  core/coreutils core/findutils core/grep core/sed core/bash
  core/hab-pkg-dockerize
)
pkg_bin_dirs=(bin)

_bins=($pkg_name)

# TODO fn: use `pkg_version()` form
pkg_version=$(cat "$PLAN_CONTEXT/../../VERSION")

do_build() {
  local bin
  for bin in "${_bins[@]}"; do
    cp -v "$PLAN_CONTEXT/bin/${bin}.sh" "$CACHE_PATH/${bin}"

    # Use the Bash from our dependency list as the shebang. Also, embed the
    # release version of the program.
    sed \
      -e "s,#!/bin/bash$,#!$(pkg_path_for bash)/bin/bash," \
      -e "s,@author@,$pkg_maintainer,g" \
      -e "s,@version@,$pkg_version/$pkg_release,g" \
      -i "$CACHE_PATH/$bin"
  done
}

do_install() {
  local bin
  for bin in "${_bins[@]}"; do
    install -v -D "$CACHE_PATH/$bin" "$pkg_prefix/bin/$bin"
  done
}
