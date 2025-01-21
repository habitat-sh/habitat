pkg_name=hab
_pkg_distname=$pkg_name
pkg_origin=core
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('Apache-2.0')
# The result is a portable, static binary in a zero-dependency package.
pkg_deps=()
pkg_build_deps=()
pkg_bin_dirs=(bin)

bin=$_pkg_distname

pkg_version() {
  cat "$SRC_PATH/../../VERSION"
}

do_before() {
  do_default_before
  update_pkg_version
}

# shellcheck disable=2155
do_prepare() {
  # Can be either `--release` or `--debug` to determine cargo build strategy
  build_type="--release"
  build_line "Building artifacts with \`${build_type#--}' mode"

  export rustc_target="aarch64-apple-darwin"
  build_line "Setting rustc_target=$rustc_target"

  # Used by the `build.rs` program to set the version of the binaries
  export PLAN_VERSION="${pkg_version}/${pkg_release}"
  build_line "Setting PLAN_VERSION=$PLAN_VERSION"

  # Used by Cargo to use a pristine, isolated directory for all compilation
  export CARGO_TARGET_DIR="$HAB_CACHE_SRC_PATH/$pkg_dirname"
  build_line "Setting CARGO_TARGET_DIR=$CARGO_TARGET_DIR"
}

do_build() {
  pushd "$SRC_PATH" > /dev/null || exit
  # strip out mac-bootstrapper which will inject non-ARM binaries
  PATH=${PATH/":/opt/mac-bootstrapper/embedded/bin:"/:} cargo build ${build_type#--debug} --target=$rustc_target --verbose
  popd > /dev/null || exit
}

do_install() {
  install -v -D "$CARGO_TARGET_DIR"/"$rustc_target"/${build_type#--}/$bin \
    "$pkg_prefix"/bin/$bin
}

# Override the do_strip implementation that the x86_64-linux plan.sh includes
#
# TODO(SM): MacOS strip doesn't provide any of --strip-{all,debug,unnecessary}
# that we use in plan-build, so I don't believe we've ever stripped our MacOS 
# binaries.  Running `file $(which hab)` returns `regular file` which would 
# bypass the existing behavior. For now, lets always skip stripping to prevent 
# accidental breakages from changes to plan-build.  
do_strip() {
  build_line "Skipping do_strip on aarch64-darwin"
  return 0
}
