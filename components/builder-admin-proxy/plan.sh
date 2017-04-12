pkg_origin=core
pkg_name=builder-admin-proxy
pkg_description="HTTP Proxy service fronting the Habitat Builder Admin service"
pkg_version=undefined
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh"
pkg_source=nosuchfile.tar.xz
pkg_license=("Apache-2.0")
pkg_deps=(core/nginx core/curl)
pkg_build_deps=(core/git)
pkg_svc_run="nginx -c ${pkg_svc_config_path}/nginx.conf"
pkg_svc_user="root"
pkg_svc_group="root"

do_verify() {
  pushd $PLAN_CONTEXT/../.. > /dev/null
  update_pkg_version
  popd > /dev/null
}

do_begin() {
  return 0
}

do_build() {
  return 0
}

do_download() {
  return 0
}

update_pkg_version() {
  # Update the `$pkg_version` using Git to determine the value
  pkg_version="$(git rev-list master --count)"
  build_line "Version updated to $pkg_version"

  # Several metadata values get their defaults from the value of `$pkg_version`
  # so we must update these as well
  pkg_dirname=${pkg_name}-${pkg_version}
  pkg_prefix=$HAB_PKG_PATH/${pkg_origin}/${pkg_name}/${pkg_version}/${pkg_release}
  pkg_artifact="$HAB_CACHE_ARTIFACT_PATH/${pkg_origin}-${pkg_name}-${pkg_version}-${pkg_release}-${pkg_target}.${_artifact_ext}"
  if [[ "$CACHE_PATH" == "$SRC_PATH" ]]; then
    local update_src_path=true
  fi
  CACHE_PATH="$HAB_CACHE_SRC_PATH/$pkg_dirname"
  if [[ "${update_src_path:-}" == true ]]; then
    SRC_PATH="$CACHE_PATH"
  fi
}

do_install() {
  return 0
}

do_prepare() {
  return 0
}

do_unpack() {
  return 0
}
