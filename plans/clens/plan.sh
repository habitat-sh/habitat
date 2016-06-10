pkg_name=clens
pkg_origin=core
pkg_version=0.7.0
pkg_license=('isc')
pkg_maintainer="The Habitat Contributors <humans@habitat.sh>"
pkg_source=http://s3-us-west-2.amazonaws.com/fnichol-lfs-tools/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=064ac9954d38633e2cff6b696fd049dedc3e90b79acffbee1a87754bcf604267
pkg_deps=(core/glibc)
pkg_build_deps=(core/coreutils core/diffutils core/patch core/make core/gcc core/libbsd)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)

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
  pkg_build_deps=(core/gcc core/coreutils core/diffutils core/make core/patch core/libbsd)
fi
