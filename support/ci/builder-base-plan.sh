# Despite the naming of this file and these functions, this file is
# not inherently related to Builder; that is simply an artifact of the
# history of this repository. This file should ultimately be used by
# all the Rust-based packages in this repository, since many of them
# duplicate the code here.

pkg_version() {
  git rev-list master --count
}

do_before() {
  do_builder_before
}

do_prepare() {
  do_builder_prepare
}

do_build() {
  do_builder_build
}

do_install() {
  do_builder_install
}

do_strip() {
  return 0
}

do_builder_before() {
  update_pkg_version
}

do_builder_build() {
  pushd "$PLAN_CONTEXT"/.. || exit
  cargo build "${builder_build_type#--debug}" --target="$rustc_target" --verbose
  popd || exit
}

do_builder_install() {
  # shellcheck disable=2154
  install -v -D "$CARGO_TARGET_DIR/$rustc_target/${builder_build_type#--}/$bin" \
    "$pkg_prefix/bin/$bin"
}

# shellcheck disable=2154
do_builder_prepare() {
  export builder_build_type="${builder_build_type:---release}"
  # Can be either `--release` or `--debug` to determine cargo build strategy
  build_line "Building artifacts with \`${builder_build_type#--}' mode"

  export rustc_target="x86_64-unknown-linux-gnu"
  build_line "Setting rustc_target=$rustc_target"

  # Used by the `build.rs` program to set the version of the binaries
  export PLAN_VERSION="${pkg_version}/${pkg_release}"
  build_line "Setting PLAN_VERSION=$PLAN_VERSION"

  # Used by Cargo to use a pristine, isolated directory for all compilation
  export CARGO_TARGET_DIR="$HAB_CACHE_SRC_PATH/$pkg_dirname"
  build_line "Setting CARGO_TARGET_DIR=$CARGO_TARGET_DIR"
}
