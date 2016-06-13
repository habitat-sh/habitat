pkg_name=hab-builder-api
pkg_origin=core
pkg_version=0.6.0
pkg_maintainer="Jamie Winsor <reset@chef.io>"
pkg_license=('apachev2')
pkg_source=nosuchfile.tar.gz
pkg_bin_dirs=(bin)
pkg_deps=(core/glibc core/openssl core/coreutils core/gcc-libs core/zeromq core/libsodium core/libarchive)
pkg_build_deps=(core/protobuf core/protobuf-rust core/coreutils core/cacerts core/rust core/gcc
                core/pkg-config core/node core/phantomjs)
pkg_expose=(9636)
srv_bin="bldr-api"
pkg_svc_run="bin/$srv_bin start -c ${pkg_svc_path}/config.toml"

do_build() {
  pushd $HAB_CACHE_SRC_PATH/ui-$pkg_name-$pkg_version > /dev/null
  export HOME=$HAB_CACHE_SRC_PATH
  npm install
  for b in node_modules/.bin/*; do
    echo $b
    fix_interpreter $(readlink -f -n $b) core/coreutils bin/env
  done
  npm run postinstall
  npm run dist
  popd > /dev/null

  # Used by the `build.rs` program to set the version of the binaries
  export PLAN_VERSION="${pkg_version}/${pkg_release}"
  build_line "Setting PLAN_VERSION=$PLAN_VERSION"

  # Used by Cargo to fetch registries/crates/etc.
  export SSL_CERT_FILE=$(pkg_path_for cacerts)/ssl/cert.pem
  build_line "Setting SSL_CERT_FILE=$SSL_CERT_FILE"

  export rustc_target="debug"
  build_line "Setting rustc_target=$rustc_target"

  export LIBARCHIVE_LIB_DIR=$(pkg_path_for libarchive)/lib
  export LIBARCHIVE_INCLUDE_DIR=$(pkg_path_for libarchive)/include
  export OPENSSL_LIB_DIR=$(pkg_path_for openssl)/lib
  export OPENSSL_INCLUDE_DIR=$(pkg_path_for openssl)/include
  export PROTOBUF_PREFIX=$(pkg_path_for protobuf)
  export SODIUM_LIB_DIR=$(pkg_path_for libsodium)/lib
  export LIBZMQ_PREFIX=$(pkg_path_for zeromq)

  pushd $PLAN_CONTEXT/.. > /dev/null
  cargo clean --target=$rustc_target --verbose
  cargo build \
    -j $(nproc) \
    --verbose
  popd > /dev/null
}

do_install() {
  cp -vR $HAB_CACHE_SRC_PATH/ui-$pkg_name-$pkg_version/dist $pkg_prefix/static
  install -v -D $PLAN_CONTEXT/../target/$rustc_target/$srv_bin $pkg_prefix/bin/$srv_bin
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
  rm -Rdf $HAB_CACHE_SRC_PATH/ui-$pkg_name-$pkg_version
  cp -ra $PLAN_CONTEXT/../../builder-web $HAB_CACHE_SRC_PATH/ui-$pkg_name-$pkg_version
  rm -Rdf $HAB_CACHE_SRC_PATH/ui-$pkg_name-$pkg_version/node_modules
}
