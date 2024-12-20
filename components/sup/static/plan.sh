# shellcheck disable=2034
PLAN_CONTEXT=$(dirname "$PLAN_CONTEXT") source ../plan.sh

pkg_name=hab-sup-static
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_deps=(core/busybox-static)
pkg_build_deps=(
  core/coreutils core/cacerts core/rust/"$(tail -n 1 "$SRC_PATH/../../rust-toolchain"  | cut -d'"' -f 2)" core/gcc
)

do_begin() {
  # Set the parent directory as the "root" of this plan.
  PLAN_CONTEXT=$(abspath ..)
}

# shellcheck disable=2155
do_prepare() {
  _common_prepare

  export rustc_target="x86_64-unknown-linux-musl"
  build_line "Setting rustc_target=$rustc_target"

  # Used to find libgcc_s.so.1 when compiling `build.rs` in dependencies. Since
  # this used only at build time, we will use the version found in the gcc
  # package proper--it won't find its way into the final binaries.
  export LD_LIBRARY_PATH=$(pkg_path_for gcc)/lib
  build_line "Setting LD_LIBRARY_PATH=$LD_LIBRARY_PATH"

  # rust 1.46.0 enabled Position Independent Executables(PIE) for x86_64-unknown-linux-musl.
  # This causes the compiled binary to segfault when building with GCC versions that
  # support it. While we should investigate if there is something in the way we compile
  # GCC which causes this. Setting relocation-model to static suppresses PIE.
  export RUSTFLAGS='-C relocation-model=static'
}
