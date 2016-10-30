pkg_name={{ pkg_name }}
pkg_origin={{ pkg_origin }}
pkg_version=0.0.1
pkg_description="The package description"
# Please choose a license from http://spdx.org/licenses/
pkg_license=('Apache-2.0')
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_source=http://some_source_url/releases/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=TODO
pkg_bin_dirs=(bin)
pkg_build_deps=(core/make core/gcc)
pkg_deps=(core/glibc)
