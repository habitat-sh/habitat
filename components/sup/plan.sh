# shellcheck disable=2034,2154
pkg_name=hab-sup
_pkg_distname=$pkg_name
pkg_origin=core
pkg_version=$(cat "$PLAN_CONTEXT/../../VERSION")
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('Apache-2.0')
pkg_deps=(core/busybox-static/1.24.2/20170513215502
          core/glibc/2.22/20170513201042
          core/gcc-libs/5.2.0/20170513212920
          core/libarchive/3.3.2/20171018164107
          core/libsodium/1.0.13/20170905223149
          core/openssl/1.0.2l/20171014213633
          core/zeromq/4.2.5/20180407102804)
pkg_build_deps=(core/coreutils/8.25/20170513213226
                core/cacerts/2017.09.20/20171014212239
                core/rust
                core/gcc/5.2.0/20170513202244
                core/raml2html/6.3.0/20180409195740)
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

# shellcheck disable=2155
do_prepare() {
  _common_prepare

  export rustc_target="x86_64-unknown-linux-gnu"
  build_line "Setting rustc_target=$rustc_target"

  export LIBARCHIVE_LIB_DIR=$(pkg_path_for libarchive)/lib
  export LIBARCHIVE_INCLUDE_DIR=$(pkg_path_for libarchive)/include
  export OPENSSL_LIB_DIR=$(pkg_path_for openssl)/lib
  export OPENSSL_INCLUDE_DIR=$(pkg_path_for openssl)/include
  export SODIUM_LIB_DIR=$(pkg_path_for libsodium)/lib
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
  build_line "Setting LD_LIBRARY_PATH=$LD_LIBRARY_PATH"
}

do_build() {
  export LIBRARY_PATH=$LIBZMQ_PREFIX/lib
  pushd "$PLAN_CONTEXT" > /dev/null
  cargo build ${build_type#--debug} --target=$rustc_target --verbose --no-default-features \
    --features apidocs
  popd > /dev/null
}

do_install() {
  install -v -D "$CARGO_TARGET_DIR"/$rustc_target/${build_type#--}/$bin \
    "$pkg_prefix"/bin/$bin
}

do_strip() {
  if [[ "$build_type" != "--debug" ]]; then
    do_default_strip
  fi
}
