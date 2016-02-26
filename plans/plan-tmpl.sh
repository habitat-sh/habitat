# Template plan.sh
pkg_name=PACKAGE
pkg_version=0.0.0
pkg_origin=chef
pkg_license=('PACKAGE')
pkg_source=http://example.com/${pkg_name}-${pkg_version}.tar.gz
pkg_filename=${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=01ba4719c80b6fe911b091a7c05124b64eeece964e09c058ef8f9805daca546b
pkg_gpg_key=3853DA6B
pkg_deps=(chef/glibc)
pkg_lib_dirs=(lib)
pkg_include_dirs=(include)
