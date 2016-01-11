pkg_name=node
pkg_derivation=chef
pkg_version=4.2.4
pkg_license=('MIT')
pkg_maintainer="Dave Parfitt <dparfitt@chef.io>"
pkg_source=https://nodejs.org/dist/${pkg_version}/${pkg_name}-v${pkg_version}.tar.gz
# the archive contains a 'v' version # prefix, but the default value of
# pkg_version is node-4.2.4 (without the v). This tweak makes bldr-build happy
pkg_dirname=node-v${pkg_version}
pkg_shasum=4ee244ffede7328d9fa24c3024787e71225b7abaac49fe2b30e68b27460c10ec
pkg_gpg_key=3853DA6B
pkg_deps=(chef/glibc)
pkg_lib_dirs=(lib)
pkg_include_dirs=(include)
