# shellcheck disable=2034,2154
pkg_name=test-probe
pkg_origin=habitat-testing
pkg_version="0.1.0"
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=("Apache-2.0")
pkg_bin_dirs=(bin)
pkg_deps=(core/glibc
          core/openssl
          core/gcc-libs
          core/procps-ng
          core/bash)
pkg_build_deps=(core/coreutils
                core/rust
                core/gcc
                core/git
                core/make)
pkg_binds_optional=(
    [thing_with_a_port]="port"
)

bin="test-probe"

do_prepare() {
  export rustc_target="x86_64-unknown-linux-gnu"
  build_line "Setting rustc_target=$rustc_target"

  # Used by Cargo to use a pristine, isolated directory for all compilation
  export CARGO_TARGET_DIR="$HAB_CACHE_SRC_PATH/$pkg_dirname"
  build_line "Setting CARGO_TARGET_DIR=$CARGO_TARGET_DIR"
}

do_build() {
    (
        cd "$PLAN_CONTEXT"/.. || exit
        cargo build --target="$rustc_target" --verbose
    )
}

do_install() {
  install -v -D "$CARGO_TARGET_DIR/$rustc_target/debug/$bin" \
    "$pkg_prefix/bin/$bin"
}

do_strip() {
    return 0
}
