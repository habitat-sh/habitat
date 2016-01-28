source ../gnupg/plan.sh

pkg_name=gnupg-static
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_dirname=${pkg_distname}-${pkg_version}

# Throw the run deps into build deps as this will be static
pkg_build_deps=("${pkg_deps[@]}" "${pkg_build_deps[@]}")
# Empty out the run deps array
pkg_deps=()

pkg_gpg_key=3853DA6B

do_prepare() {
  LDFLAGS="-static $LDFLAGS"
  build_line "Updating LDFLAGS=$LDFLAGS"
}
