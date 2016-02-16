pkg_name=clens
pkg_derivation=chef
pkg_version=0.7.0
pkg_license=('isc')
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_source=https://opensource.conformal.com/snapshots/$pkg_name/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=064ac9954d38633e2cff6b696fd049dedc3e90b79acffbee1a87754bcf604267
pkg_deps=(chef/glibc)
pkg_build_deps=(chef/coreutils chef/diffutils chef/patch chef/make chef/gcc chef/libbsd)
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


# ----------------------------------------------------------------------------
# **NOTICE:** What follows are implementation details required for building a
# first-pass, "stage1" toolchain and environment. It is only used when running
# in a "stage1" Studio and can be safely ignored by almost everyone. Having
# said that, it performs a vital bootstrapping process and cannot be removed or
# significantly altered. Thank you!
# ----------------------------------------------------------------------------
if [[ "$STUDIO_TYPE" = "stage1" ]]; then
  pkg_build_deps=(chef/gcc chef/coreutils chef/diffutils chef/make chef/patch chef/libbsd)
fi
