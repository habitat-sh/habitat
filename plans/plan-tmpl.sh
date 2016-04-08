# Template plan.sh
pkg_origin=chef
pkg_name=PACKAGE
pkg_version=0.0.1
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=()
pkg_source=http://example.com/${pkg_name}-${pkg_version}.tar.xz
pkg_shasum=sha256sum
pkg_deps=(chef/glibc)
pkg_build_deps=(chef/coreutils)
pkg_bin_dirs=(bin)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)
pkg_gpg_key=3853DA6B
