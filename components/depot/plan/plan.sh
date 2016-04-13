pkg_name=hab-depot
pkg_origin=chef
pkg_version=0.4.0
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('apachev2')
pkg_source=nosuchfile.tar.gz
pkg_bin_dirs=(bin)
pkg_deps=(chef/glibc chef/openssl chef/gcc-libs chef/libarchive chef/libsodium)
pkg_build_deps=(chef/coreutils chef/cacerts chef/rust chef/gcc chef/libsodium)
pkg_service_run="bin/hab-depot start"

do_build() {
  pushd $PLAN_CONTEXT/.. > /dev/null
  cargo clean
  env OPENSSL_LIB_DIR=$(pkg_path_for chef/openssl)/lib \
      OPENSSL_INCLUDE_DIR=$(pkg_path_for chef/openssl)/include \
      LIBARCHIVE_LIB_DIR=$(pkg_path_for chef/libarchive)/lib \
      LIBARCHIVE_INCLUDE_DIR=$(pkg_path_for chef/libarchive)/include \
      SSL_CERT_FILE=$(pkg_path_for chef/cacerts)/ssl/cert.pem \
      SODIUM_LIB_DIR=$(pkg_path_for chef/libsodium)/lib
      cargo build --verbose
  popd > /dev/null
}

do_install() {
  install -v -D $PLAN_CONTEXT/../target/debug/$pkg_name $pkg_prefix/bin/$pkg_name
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

do_prepare() {
  return 0
}
