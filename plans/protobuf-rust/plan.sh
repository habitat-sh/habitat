pkg_name=protobuf-rust
pkg_origin=core
pkg_version=1.0.18
pkg_license=('BSD')
pkg_source=nosuchfile.tar.gz
pkg_bin_dirs=(bin)
pkg_deps=(core/glibc core/gcc)
pkg_build_deps=(core/rust core/gcc core/cacerts core/protobuf)

do_build() {
  env SSL_CERT_FILE=$(pkg_path_for cacerts)/ssl/cert.pem \
      PROTOBUF_PREFIX=$(pkg_path_for protobuf) \
      cargo install protobuf --root $pkg_prefix --vers $pkg_version -j$(nproc) --verbose
}

do_install() {
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

do_prepare() {
  return 0
}
