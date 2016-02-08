pkg_name=node
pkg_derivation=chef
pkg_version=4.2.6
pkg_license=('MIT')
pkg_maintainer="Dave Parfitt <dparfitt@chef.io>"
pkg_source=https://nodejs.org/dist/v${pkg_version}/${pkg_name}-v${pkg_version}.tar.gz
# the archive contains a 'v' version # prefix, but the default value of
# pkg_version is node-4.2.6 (without the v). This tweak makes bldr-build happy
pkg_dirname=node-v${pkg_version}
pkg_shasum=ea5e357db8817052b17496d607c719d809ed1383e8fcf7c8ffc5214e705aefdd
pkg_gpg_key=3853DA6B
pkg_deps=(chef/glibc)
pkg_lib_dirs=(lib)
pkg_include_dirs=(include)
pkg_binary_path=(bin)
