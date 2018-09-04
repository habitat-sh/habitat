# shellcheck disable=SC2034
pkg_origin=habitat
pkg_name=sup-proxy
pkg_description="HTTP Proxy service fronting the Habitat Supervisor API service"
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=("Apache-2.0")
pkg_deps=(core/nginx core/cacerts)
pkg_build_deps=(core/git)
pkg_svc_user="root"
# shellcheck disable=2154
pkg_svc_run="nginx -c ${pkg_svc_config_path}/nginx.conf"

pkg_version() {
  git rev-list master --count
}

do_before() {
  update_pkg_version
}

do_build() {
    return 0
}

do_install() {
  return 0
}
