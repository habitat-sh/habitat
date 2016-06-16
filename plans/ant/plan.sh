pkg_origin=core
pkg_name=ant
pkg_version=1.9.7
pkg_maintainer='The Habitat Maintainers <humans@habitat.sh>'
pkg_license=('Apache 2.0')
pkg_source=http://apache.osuosl.org//ant/binaries/apache-${pkg_name}-${pkg_version}-bin.tar.gz
pkg_shasum=1d0b808fe82cce9bcc167f8dbb7c7e89c1d7f7534c0d9c64bf615ec7c3e6bfe5
pkg_deps=(
  core/server-jre
)
pkg_build_deps=(
)
pkg_bin_dirs=(bin)

do_build() {
  return 0
}

do_install() {
  return 0
}
