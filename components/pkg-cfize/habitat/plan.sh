# shellcheck disable=2154
pkg_name=hab-pkg-cfize
pkg_origin=core
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('Apache-2.0')
pkg_description="Habitat Cloud Foundry image exporter"
pkg_upstream_url="https://github.com/habitat-sh/habitat"

# Docker is actually also pulled in by hab-pkg-export-docker, but we
# explicitly call it here, so it's a dependency. Docker doesn't have
# any dependencies, so we can unpin here without worrying about
# getting dependency conflicts.
#
# We're pinning the other dependencies to their pre base-plans refresh
# versions for the time being for explicitness, due to a bug in how
# `hab pkg install` works in the context of our release pipeline.
#
# It's a bit of a moot point, though, since Docker's not going to run
# on older kernels anyway.
pkg_deps=(core/coreutils
          core/findutils
          core/grep
          core/gawk
          core/hab-pkg-export-docker
          core/docker)
pkg_bin_dirs=(bin)

_bins=("$pkg_name")

# TODO fn: use `pkg_version()` form
pkg_version=$(cat "$SRC_PATH/../../VERSION")

do_prepare() {
  do_default_prepare

  # This builds up a reasonable runtime `PATH` for the program which includes
  # all direct and indirect runtime dependencies' `PATH` entries.
  local p
  p=()
  for tdep in "${pkg_tdeps_resolved[@]}"; do
    if [[ -f "$tdep/PATH" ]]; then
      mapfile -t p < <(cat "$tdep/PATH")
    fi
  done
  _runtime_path="$(join_by : "${p[@]}")"
}

do_build() {
  local bin
  for bin in "${_bins[@]}"; do
    cp -v "$SRC_PATH/bin/${bin}.sh" "$CACHE_PATH/${bin}"

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
    # Wrap the program with a script which sets the runtime `PATH` for the
    # program so that it can find all of the programs it calls out to, such as
    # `docker`, `hab-pkg-export-docker`, etc.
    _wrap_binary "$pkg_prefix/bin/$bin"
  done
}

_wrap_binary() {
  local bin="$1"
  build_line "Adding wrapper $bin to ${bin}.real"
  mv -v "$bin" "${bin}.real"
  cat <<EOF > "$bin"
#!$(pkg_path_for bash)/bin/bash
set -e
if test -n "\$DEBUG"; then set -x; fi

export PATH="$_runtime_path:\${PATH:-}"

exec ${bin}.real \$@
EOF
  chmod -v 755 "$bin"
}
