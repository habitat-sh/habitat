pkg_name=simple_service
pkg_origin=test
pkg_version=0.0.1
pkg_license=('Apache2')
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_source=nosuchfile.tar.gz
pkg_bin_dirs=(bin)
pkg_deps=()
pkg_svc_run="bin/simple_service"

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
  cp -r $PLAN_CONTEXT/bin $pkg_prefix
  cp -r /src/components/sup/target/debug/hab-sup $pkg_prefix/bin
  chmod 755 $pkg_prefix/bin
  chmod 755 $pkg_prefix/bin/*
}
