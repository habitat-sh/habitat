pkg_name=clens
pkg_derivation=chef
pkg_version=0.7.0
pkg_license=('isc')
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_source=https://opensource.conformal.com/snapshots/$pkg_name/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=064ac9954d38633e2cff6b696fd049dedc3e90b79acffbee1a87754bcf604267
pkg_deps=(chef/glibc)
pkg_build_deps=(chef/gcc chef/coreutils chef/sed chef/bison chef/flex chef/grep chef/bash chef/gawk chef/libtool chef/diffutils chef/findutils chef/xz chef/gettext chef/gzip chef/make chef/patch chef/texinfo chef/util-linux chef/wget chef/libbsd)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)
pkg_gpg_key=3853DA6B

do_build() {
  mkdir -pv obj
  make LOCALBASE=$pkg_prefix
}

do_install() {
  make LOCALBASE=$pkg_prefix install
}
