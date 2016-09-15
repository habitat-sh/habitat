pkg_origin=core
pkg_name=builder-api-proxy
pkg_description="HTTP Proxy service fronting the Habitat Builder API service"
pkg_version=$(cat "${PLAN_CONTEXT}/../../VERSION")
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh"
pkg_source=nosuchfile.tar.xz
pkg_license=("Apache-2.0")

# we depend on a specific version of nginx until we can build this
# with a released version of hab that fixes the dependency version
# sorting bug: https://github.com/habitat-sh/habitat/pull/1221
pkg_deps=(core/nginx/1.10.1 core/curl)
pkg_svc_run="nginx -c ${pkg_svc_config_path}/nginx.conf"
# nginx is configured to drop privileges to hab:hab
pkg_svc_user="root"
pkg_svc_group="root"

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

do_verify() {
  return 0
}
