# shellcheck disable=2034,2154
pkg_name=hab-sup
_pkg_distname=$pkg_name
pkg_origin=core
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('Apache-2.0')
pkg_deps=(core/busybox-static
          core/glibc
          core/gcc-libs
          core/openssl
          core/zeromq)
pkg_build_deps=(core/coreutils
                core/cacerts
                core/make
                core/rust/"$(cat "$SRC_PATH/../../rust-toolchain")"
                core/gcc
                core/raml2html
                core/protobuf)
pkg_bin_dirs=(bin)

bin=$_pkg_distname

_common_prepare() {
  do_default_prepare

  # Can be either `--release` or `--debug` to determine cargo build strategy
  build_type="--release"
  build_line "Building artifacts with \`${build_type#--}' mode"

  # Used by the `build.rs` program to set the version of the binaries
  export PLAN_VERSION="${pkg_version}/${pkg_release}"
  build_line "Setting PLAN_VERSION=$PLAN_VERSION"

  # Used to set the active package target for the binaries at build time
  export PLAN_PACKAGE_TARGET="$pkg_target"
  build_line "Setting PLAN_PACKAGE_TARGET=$PLAN_PACKAGE_TARGET"

  if [ -z "$HAB_CARGO_TARGET_DIR" ]; then
    # Used by Cargo to use a pristine, isolated directory for all compilation
    export CARGO_TARGET_DIR="$HAB_CACHE_SRC_PATH/$pkg_dirname"
  else
    export CARGO_TARGET_DIR="$HAB_CARGO_TARGET_DIR"
  fi
  build_line "Setting CARGO_TARGET_DIR=$CARGO_TARGET_DIR"
}

pkg_version() {
  cat "$SRC_PATH/../../VERSION"
}

do_before() {
  do_default_before
  update_pkg_version
}

# shellcheck disable=2155
do_prepare() {
  _common_prepare

  export rustc_target="x86_64-unknown-linux-gnu"
  build_line "Setting rustc_target=$rustc_target"

  export OPENSSL_LIB_DIR=$(pkg_path_for openssl)/lib
  export OPENSSL_INCLUDE_DIR=$(pkg_path_for openssl)/include
  export LIBZMQ_PREFIX=$(pkg_path_for zeromq)

  # TODO (CM, FN): This is not needed to build the supervisor,
  # strictly speaking, but is instead a work-around for how we are
  # currently building packages in Travis; we hypothesize that the
  # build.rs program for habitat_http_client, built during a static
  # hab package build, is being inadvertently used here. Without gcc
  # libs on the LD_LIBRARY_PATH, the program can't find
  # libgcc_s.so.1. This is merely a bandaid until we can overhaul our
  # CI pipeline properly.
  #
  # Used to find libgcc_s.so.1 when compiling `build.rs` in dependencies. Since
  # this used only at build time, we will use the version found in the gcc
  # package proper--it won't find its way into the final binaries.
  export LD_LIBRARY_PATH=$(pkg_path_for gcc)/lib

  # Prost (our Rust protobuf library) embeds a `protoc` binary, but
  # it's dynamically linked, which means it won't work in a
  # Studio. Prost does allow us to override that, though, so we can
  # just use our Habitat package by setting these two environment
  # variables.
  export PROTOC="$(pkg_path_for protobuf)/bin/protoc"
  export PROTOC_INCLUDE="$(pkg_path_for protobuf)/include"

  build_line "Setting LD_LIBRARY_PATH=$LD_LIBRARY_PATH"
}

do_build() {
  export LIBRARY_PATH=$LIBZMQ_PREFIX/lib

  pushd "$SRC_PATH" > /dev/null || exit
  cargo build ${build_type#--debug} --target=$rustc_target --verbose --no-default-features \
    --features apidocs
  popd > /dev/null || exit
}

do_install() {
  install -v -D "$CARGO_TARGET_DIR"/$rustc_target/${build_type#--}/$bin \
    "$pkg_prefix"/bin/$bin
}

do_strip() {
  if [[ "$build_type" != "--debug" ]]; then
    strip --strip-debug "$pkg_prefix"/bin/$bin
  fi
}
