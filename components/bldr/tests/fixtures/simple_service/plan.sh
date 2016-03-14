pkg_name=simple_service
pkg_origin=test
pkg_version=0.0.1
pkg_license=('Apache2')
pkg_maintainer="Adam Jacob <adam@chef.io>"
pkg_source=nosuchfile.tar.gz
pkg_gpg_key=3853DA6B
pkg_binary_path=(bin)
pkg_deps=()
pkg_service_run="bin/simple_service"

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
  cp -r /src/target/debug/bldr $pkg_prefix/bin
  chmod 755 $pkg_prefix/bin
  chmod 755 $pkg_prefix/bin/*
  return 0
}

