pkg_name=hab-depot
pkg_origin=core
pkg_version=0.8.0
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('apachev2')
pkg_source=nosuchfile.tar.gz
pkg_deps=(core/glibc core/gcc-libs core/libarchive core/libsodium core/openssl core/zeromq)
pkg_build_deps=(core/coreutils core/cacerts core/rust core/gcc core/pkg-config)
pkg_bin_dirs=(bin)
srv_bin="hab-depot"
pkg_svc_run="bin/$srv_bin start -c ${pkg_svc_path}/config.toml"

do_prepare() {
  # Used by the `build.rs` program to set the version of the binaries
  export PLAN_VERSION="${pkg_version}/${pkg_release}"
  build_line "Setting PLAN_VERSION=$PLAN_VERSION"

  # Used by Cargo to fetch registries/crates/etc.
  export SSL_CERT_FILE=$(pkg_path_for cacerts)/ssl/cert.pem
  build_line "Setting SSL_CERT_FILE=$SSL_CERT_FILE"

  export rustc_target="x86_64-unknown-linux-gnu"
  build_line "Setting rustc_target=$rustc_target"

  export LIBARCHIVE_LIB_DIR=$(pkg_path_for libarchive)/lib
  export LIBARCHIVE_INCLUDE_DIR=$(pkg_path_for libarchive)/include
  export OPENSSL_LIB_DIR=$(pkg_path_for openssl)/lib
  export OPENSSL_INCLUDE_DIR=$(pkg_path_for openssl)/include
  export SODIUM_LIB_DIR=$(pkg_path_for libsodium)/lib
  export LIBZMQ_PREFIX=$(pkg_path_for zeromq)
}

do_build() {
  pushd $PLAN_CONTEXT > /dev/null
  cargo clean --target=$rustc_target --verbose
  cargo build \
    -j $(nproc) \
    --target=$rustc_target \
    --verbose
  popd > /dev/null
}

do_install() {
  install -v -D $PLAN_CONTEXT/../target/$rustc_target/debug/$srv_bin \
    $pkg_prefix/bin/$srv_bin
}

do_strip() {
  return 0
}

do_download() {
  return 0
}

do_verify() {
  return 0
}

do_unpack() {
  return 0
}
