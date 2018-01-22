pkg_name=hab-butterfly
_pkg_distname=$pkg_name
pkg_origin=core
pkg_version=$(cat "$PLAN_CONTEXT/../../VERSION")
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('Apache-2.0')
pkg_source=nosuchfile.tar.gz
pkg_deps=(
  core/busybox-static
  core/glibc core/gcc-libs core/libarchive core/libsodium core/openssl core/zeromq
)
pkg_build_deps=(core/coreutils core/cacerts core/rust core/gcc core/protobuf)
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

  if [ -z "$HAB_CARGO_TARGET_DIR" ]; then
    # Used by Cargo to use a pristine, isolated directory for all compilation
    export CARGO_TARGET_DIR="$HAB_CACHE_SRC_PATH/$pkg_dirname"
  else
    export CARGO_TARGET_DIR="$HAB_CARGO_TARGET_DIR"
  fi
  build_line "Setting CARGO_TARGET_DIR=$CARGO_TARGET_DIR"
}

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
  export PROTOBUF_PREFIX=$(pkg_path_for protobuf)
}

do_build() {
  export LIBRARY_PATH=$LIBZMQ_PREFIX/lib
  build_line "Setting LIBRARY_PATH=$LIBRARY_PATH"
  pushd $PLAN_CONTEXT > /dev/null
  cargo build ${build_type#--debug} --target=$rustc_target --verbose
  popd > /dev/null
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

# Turn the remaining default phases into no-ops
do_download() {
  return 0
}

do_verify() {
  return 0
}

do_unpack() {
  return 0
}

