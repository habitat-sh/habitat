pkg_origin=core
pkg_name=builder-api-proxy
pkg_description="HTTP Proxy service fronting the Habitat Builder API service"
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh"
pkg_license=("Apache-2.0")
pkg_deps=(core/nginx core/curl core/coreutils)
pkg_build_deps=(core/node core/gcc core/git core/tar core/phantomjs core/python2 core/make)
pkg_svc_user="root"
pkg_svc_group="root"
pkg_exports=(
  [port]=http.listen_port
  [url]=app_url
)
pkg_binds=(
  [http]="port"
)
pkg_exposes=(port)

pkg_version() {
  git rev-list master --count
}

do_before() {
  update_pkg_version
}

do_unpack() {
  pushd "$PLAN_CONTEXT/../builder-web" > /dev/null
  { git ls-files; git ls-files --exclude-standard --others; } \
  | _tar_pipe_app_cp_to "${HAB_CACHE_SRC_PATH}"
  popd > /dev/null
}

do_build() {
  pushd $HAB_CACHE_SRC_PATH > /dev/null
  export HOME=$HAB_CACHE_SRC_PATH
  export PATH=./node_modules/.bin:$PATH
  npm install
  for b in node_modules/.bin/*; do
    fix_interpreter $(readlink -f -n $b) core/coreutils bin/env
  done
  npm run dist
  rm -rf dist/node_modules
  popd > /dev/null
}

do_install() {
  cp -a "${HAB_CACHE_SRC_PATH}/dist/." "${pkg_prefix}/app/"
}

_tar_pipe_app_cp_to() {
  local dst_path tar
  dst_path="$1"
  tar="$(pkg_path_for tar)/bin/tar"
  "$tar" -cp \
  --owner=root:0 \
  --group=root:0 \
  --no-xattrs \
  --exclude-backups \
  --exclude-vcs \
  --exclude='habitat' \
  --files-from=- \
  -f - \
  | "$tar" -x \
  -C "$dst_path" \
  -f -
}
