pkg_name=docker
pkg_origin=core
pkg_version=1.11.1
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('Apache-2')
pkg_source=https://get.docker.com/builds/Linux/x86_64/${pkg_name}-${pkg_version}.tgz
pkg_shasum=893e3c6e89c0cd2c5f1e51ea41bc2dd97f5e791fcfa3cee28445df277836339d
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
