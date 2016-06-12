pkg_name=hab
pkg_distname=$pkg_name
pkg_origin=core
pkg_version=0.6.0
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('Apache-2.0')
pkg_source=nosuchfile.tar.gz
# The result is a portible, static binary in a zero-dependency package.
pkg_deps=()
pkg_build_deps=(
  core/musl core/zlib-musl core/xz-musl core/bzip2-musl core/libarchive-musl
  core/openssl-musl core/libsodium-musl
  core/coreutils core/cacerts core/rust core/gcc
)
pkg_bin_dirs=(bin)

program=$pkg_distname

_common_prepare() {
  do_default_prepare

  # Used by the `build.rs` program to set the version of the binaries
  export PLAN_VERSION="${pkg_version}/${pkg_release}"
  build_line "Setting PLAN_VERSION=$PLAN_VERSION"

  # Used by Cargo to fetch registries/crates/etc.
  export SSL_CERT_FILE=$(pkg_path_for cacerts)/ssl/cert.pem
  build_line "Setting SSL_CERT_FILE=$SSL_CERT_FILE"

  # Used to find libgcc_s.so.1 when compiling `build.rs` in dependencies. Since
  # this used only at build time, we will use the version found in the gcc
  # package proper--it won't find its way into the final binaries.
  export LD_LIBRARY_PATH=$(pkg_path_for gcc)/lib
  build_line "Setting LD_LIBRARY_PATH=$LD_LIBRARY_PATH"
}

do_prepare() {
  _common_prepare

  export rustc_target="x86_64-unknown-linux-musl"
  build_line "Setting rustc_target=$rustc_target"

  la_ldflags="-L$(pkg_path_for zlib-musl)/lib -lz"
  la_ldflags="$la_ldflags -L$(pkg_path_for xz-musl)/lib -llzma"
  la_ldflags="$la_ldflags -L$(pkg_path_for bzip2-musl)/lib -lbz2"
  la_ldflags="$la_ldflags -L$(pkg_path_for openssl-musl)/lib -lssl -lcrypto"

  export LIBARCHIVE_LIB_DIR=$(pkg_path_for libarchive-musl)/lib
  export LIBARCHIVE_INCLUDE_DIR=$(pkg_path_for libarchive-musl)/include
  export LIBARCHIVE_LDFLAGS="$la_ldflags"
  export LIBARCHIVE_STATIC=true
  export OPENSSL_LIB_DIR=$(pkg_path_for openssl-musl)/lib
  export OPENSSL_INCLUDE_DIR=$(pkg_path_for openssl-musl)/include
  export OPENSSL_STATIC=true
  export SODIUM_LIB_DIR=$(pkg_path_for libsodium-musl)/lib
  export SODIUM_STATIC=true
}

do_build() {
  pushd $PLAN_CONTEXT > /dev/null
  cargo clean --target=$rustc_target --verbose
  cargo build \
    -j $(nproc) \
    --target=$rustc_target \
    --release \
    --verbose
  popd > /dev/null
}

do_install() {
  install -v -D $PLAN_CONTEXT/target/$rustc_target/release/$program \
    $pkg_prefix/bin/$program
}

do_strip() {
  strip $pkg_prefix/bin/$program
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
