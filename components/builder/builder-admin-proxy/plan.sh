pkg_origin=core
pkg_name=builder-admin-proxy
pkg_description="HTTP Proxy service fronting the Habitat Builder Admin service"
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh"
pkg_license=("Apache-2.0")
pkg_deps=(core/nginx core/curl)
pkg_build_deps=(core/git)
pkg_svc_run="nginx -c ${pkg_svc_config_path}/nginx.conf"
pkg_svc_user="root"
pkg_svc_group="root"
pkg_exports=(
  [port]=http.listen_port
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

do_build() {
  return 0
}

do_install() {
  return 0
}

do_unpack() {
  return 0
}
