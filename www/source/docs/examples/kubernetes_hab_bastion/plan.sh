pkg_name=hab_bastion
pkg_origin=habitat
pkg_version="0.1.0"
pkg_maintainer="irvingpop"
pkg_license=("Apache-2.0")
pkg_deps=(core/busybox-static)
pkg_svc_run="while true; do sleep 60; done"

do_build() {
  return 0
}

do_install() {
  return 0
}

