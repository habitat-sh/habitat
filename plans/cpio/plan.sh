pkg_origin=core
pkg_name=cpio
pkg_version='2.12'
pkg_maintainer='The Habitat Maintainers <humans@habitat.sh>'
pkg_license=('GPL-3.0')
pkg_source=http://ftp.gnu.org/gnu/cpio/cpio-2.12.tar.gz
pkg_shasum=08a35e92deb3c85d269a0059a27d4140a9667a6369459299d08c17f713a92e73
pkg_deps=()
pkg_build_deps=(
  core/make
  core/gcc
)
pkg_bin_dirs=(bin)
