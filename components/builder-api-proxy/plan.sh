pkg_origin=core
pkg_name=builder-api-proxy
pkg_description="HTTP Proxy service fronting the Habitat Builder API service"
pkg_version=undefined
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh"
pkg_source=nosuchfile.tar.xz
pkg_license=("Apache-2.0")
pkg_deps=(core/nginx core/curl)
pkg_build_deps=(core/git)
pkg_svc_run="nginx -c ${pkg_svc_config_path}/nginx.conf"
# nginx is configured to drop privileges to hab:hab
pkg_svc_user="root"
pkg_svc_group="root"

do_verify() {
  pushd $PLAN_CONTEXT/../../.. > /dev/null
  pkg_version=`git rev-list master --count`
  pkg_dirname="${pkg_name}-${pkg_version}"
  pkg_prefix="$HAB_PKG_PATH/${pkg_origin}/${pkg_name}/${pkg_version}/${pkg_release}"
  pkg_artifact="$HAB_CACHE_ARTIFACT_PATH/${pkg_origin}-${pkg_name}-${pkg_version}-${pkg_release}-${pkg_target}.${_artifact_ext}"
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

do_install() {
  return 0
}

do_prepare() {
  return 0
}

do_unpack() {
  return 0
}
