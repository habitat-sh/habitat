source ../gnupg/plan.sh

pkg_name=gnupg-static
pkg_maintainer="The Habitat Contributors <humans@habitat.sh>"
pkg_dirname=${pkg_distname}-${pkg_version}

# Throw the run deps into build deps as this will be static
pkg_build_deps=("${pkg_deps[@]}" "${pkg_build_deps[@]}")
# Empty out the run deps array
pkg_deps=()


do_prepare() {
  LDFLAGS="-static $LDFLAGS"
  build_line "Updating LDFLAGS=$LDFLAGS"
}
