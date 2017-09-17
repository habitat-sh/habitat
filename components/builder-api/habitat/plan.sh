source "../../../support/ci/builder-base-plan.sh"
pkg_name=builder-api
pkg_origin=core
pkg_maintainer="Jamie Winsor <reset@chef.io>"
pkg_license=('Apache-2.0')
pkg_bin_dirs=(bin)
pkg_deps=(core/glibc core/openssl core/coreutils core/gcc-libs core/zeromq core/libsodium
  core/libarchive)
pkg_build_deps=(core/protobuf core/protobuf-rust core/coreutils core/cacerts
  core/rust core/gcc core/git core/pkg-config core/node core/phantomjs core/python2 core/make)
pkg_exports=(
  [port]=http.port
  [url]=web.app_url
)
pkg_exposes=(port)
pkg_binds=(
  [router]="port"
)
bin="bldr-api"

do_prepare() {
  rm -Rdf $HAB_CACHE_SRC_PATH/ui-$pkg_name-$pkg_version
  cp -ra $PLAN_CONTEXT/../../builder-web $HAB_CACHE_SRC_PATH/ui-$pkg_name-$pkg_version
  rm -Rdf $HAB_CACHE_SRC_PATH/ui-$pkg_name-$pkg_version/node_modules

  do_builder_prepare
}

do_build() {
  pushd $HAB_CACHE_SRC_PATH/ui-$pkg_name-$pkg_version > /dev/null
  export HOME=$HAB_CACHE_SRC_PATH
  npm install
  for b in node_modules/.bin/*; do
    echo $b
    fix_interpreter $(readlink -f -n $b) core/coreutils bin/env
  done
  npm run dist
  popd > /dev/null

  do_builder_build
}

do_install() {
  cp -vR $HAB_CACHE_SRC_PATH/ui-$pkg_name-$pkg_version/dist $pkg_prefix/static

  do_builder_install
}
