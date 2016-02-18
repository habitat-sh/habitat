pkg_name=bldr
pkg_derivation=chef
pkg_version=0.4.0
pkg_license=('Apache2')
pkg_source=nosuchfile.tar.gz
pkg_gpg_key=3853DA6B
pkg_binary_path=(bin)
pkg_build_deps=(chef/coreutils chef/tar chef/patchelf chef/cacerts chef/rust chef/gcc)
pkg_deps=(chef/openssl chef/runit chef/gpgme chef/libarchive chef/libgpg-error)

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

do_build() {
  pushd $PLAN_CONTEXT
  cargo clean
  env OPENSSL_LIB_DIR=$(pkg_path_for chef/openssl)/lib \
      OPENSSL_INCLUDE_DIR=$(pkg_path_for chef/openssl)/include \
      GPGME_CONFIG=$(pkg_path_for chef/gpgme)/bin/gpgme-config \
      GPG_ERROR_CONFIG=$(pkg_path_for chef/libgpg-error)/bin/gpg-error-config \
      LIBARCHIVE_LIB_DIR=$(pkg_path_for chef/libarchive)/lib \
      LIBARCHIVE_INCLUDE_DIR=$(pkg_path_for chef/libarchive)/include \
      SSL_CERT_FILE=$(pkg_path_for chef/cacerts)/ssl/cert.pem \
      cargo build --verbose
}

do_install() {
  mkdir -p $pkg_path/bin
  cp $PLAN_CONTEXT/../../target/debug/bldr $pkg_path/bin
}

do_docker_image() {
  return 0
}

# do_docker_image() {
#   ./mkimage.sh
#   docker build -t "bldr/base:${pkg_version}-${pkg_rel}" .
#   docker tag -f bldr/base:${pkg_version}-${pkg_rel} bldr/base:latest
# }
