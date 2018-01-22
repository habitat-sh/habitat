pkg_name=airlock
pkg_origin=core
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('Apache-2.0')
# The result is a portable, static binary in a zero-dependency package.
pkg_deps=(core/shadow core/iproute2)
pkg_build_deps=(core/musl core/coreutils core/rust core/gcc core/git)
pkg_bin_dirs=(bin)

bin="$pkg_name"

pkg_version() {
  git rev-list master --count
}

do_before() {
  do_default_before
  update_pkg_version
}

_common_prepare() {
  do_default_prepare

  # Can be either `--release` or `--debug` to determine cargo build strategy
  build_type="--release"
  build_line "Building artifacts with \`${build_type#--}' mode"

  # Used by the `build.rs` program to set the version of the binaries
  export PLAN_VERSION="${pkg_version}/${pkg_release}"
  build_line "Setting PLAN_VERSION=$PLAN_VERSION"

  export CARGO_TARGET_DIR="$HAB_CACHE_SRC_PATH/$pkg_dirname"
  build_line "Setting CARGO_TARGET_DIR=$CARGO_TARGET_DIR"
}

do_prepare() {
  _common_prepare

  export rustc_target="x86_64-unknown-linux-musl"
  build_line "Setting rustc_target=$rustc_target"

  # Used to find libgcc_s.so.1 when compiling `build.rs` in dependencies. Since
  # this used only at build time, we will use the version found in the gcc
  # package proper--it won't find its way into the final binaries.
  export LD_LIBRARY_PATH=$(pkg_path_for gcc)/lib
  build_line "Setting LD_LIBRARY_PATH=$LD_LIBRARY_PATH"
}

do_build() {
  cargo build ${build_type#--debug} --target=$rustc_target --verbose
}

do_install() {
  install -v -D $CARGO_TARGET_DIR/$rustc_target/${build_type#--}/$bin \
    $pkg_prefix/bin/$bin
}

do_strip() {
  if [[ "$build_type" != "--debug" ]]; then
    do_default_strip
  fi
}
