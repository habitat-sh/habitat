pkg_name=hab-builder-sessionsrv
pkg_origin=core
pkg_version=0.4.0
pkg_maintainer="Jamie Winsor <reset@chef.io>"
pkg_license=('apachev2')
pkg_source=nosuchfile.tar.gz
pkg_bin_dirs=(bin)
pkg_deps=(chef/glibc chef/openssl chef/gcc-libs core/zeromq)
pkg_build_deps=(core/protobuf core/protobuf-rust chef/coreutils chef/cacerts chef/rust chef/gcc chef/pkg-config)
pkg_service_run="bin/bldr-session-srv ${pkg_svc_config_path}/app.cfg.toml"

do_build() {
  pushd $PLAN_CONTEXT/../../components/builder-sessionsrv > /dev/null
  cargo clean
  env OPENSSL_LIB_DIR=$(pkg_path_for chef/openssl)/lib \
      OPENSSL_INCLUDE_DIR=$(pkg_path_for chef/openssl)/include \
      SSL_CERT_FILE=$(pkg_path_for chef/cacerts)/ssl/cert.pem \
      PROTOBUF_PREFIX=$(pkg_path_for core/protobuf) \
      LIBZMQ_PREFIX=$(pkg_path_for core/zeromq) \
      cargo build -j$(nproc) --verbose
  popd > /dev/null
}

do_install() {
  install -v -D $PLAN_CONTEXT/../../components/builder-sessionsrv/target/debug/bldr-session-srv $pkg_prefix/bin/bldr-session-srv
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
