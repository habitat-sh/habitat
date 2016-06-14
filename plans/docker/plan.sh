pkg_name=docker
pkg_origin=core
pkg_version=1.11.2
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('Apache-2')
pkg_source=https://get.docker.com/builds/Linux/x86_64/${pkg_name}-${pkg_version}.tgz
pkg_shasum=8c2e0c35e3cda11706f54b2d46c2521a6e9026a7b13c7d4b8ae1f3a706fc55e1
pkg_dirname=docker
pkg_deps=()
pkg_build_deps=()
pkg_bin_dirs=(bin)

do_build() {
  return 0
}

do_install() {
  for bin in docker $(ls -1 docker-*); do
    install -v -D $bin $pkg_prefix/bin/$bin
  done
}
