pkg_name=database-server
pkg_origin=hab_test
pkg_version=0.1.0
pkg_license=('Apache2')
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_source=nosuchfile.tar.gz
pkg_svc_run="app-server"
pkg_exports=(
  [port]=srv.port
  [host]=srv.host
)

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

do_build() {
  return 0
}

do_install() {
  return 0
}
