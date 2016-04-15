pkg_name=hab-builder-api
pkg_origin=core
pkg_version=0.4.0
pkg_maintainer="Jamie Winsor <reset@chef.io>"
pkg_license=('apachev2')
pkg_source=nosuchfile.tar.gz
pkg_bin_dirs=(bin)
pkg_deps=(core/glibc core/openssl core/gcc-libs core/zeromq)
pkg_build_deps=(core/protobuf core/protobuf-rust core/coreutils core/cacerts core/rust core/gcc core/pkg-config)
pkg_service_run="bin/bldr-api ${pkg_svc_config_path}/app.cfg.toml"

do_build() {
  pushd $PLAN_CONTEXT/../../components/builder-api > /dev/null
  cargo clean
  env OPENSSL_LIB_DIR=$(pkg_path_for openssl)/lib \
      OPENSSL_INCLUDE_DIR=$(pkg_path_for openssl)/include \
      SSL_CERT_FILE=$(pkg_path_for cacerts)/ssl/cert.pem \
      PROTOBUF_PREFIX=$(pkg_path_for protobuf) \
      LIBZMQ_PREFIX=$(pkg_path_for zeromq) \
      cargo build -j$(nproc) --verbose
  popd > /dev/null
}

do_install() {
  install -v -D $PLAN_CONTEXT/../../components/builder-api/target/debug/bldr-api $pkg_prefix/bin/bldr-api
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

do_prepare() {
  return 0
}
