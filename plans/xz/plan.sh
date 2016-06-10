pkg_name=xz
pkg_distname=$pkg_name
pkg_origin=core
pkg_version=5.2.2
pkg_maintainer="The Habitat Contributors <humans@habitat.sh>"
pkg_license=('gpl2+' 'lgpl2+')
pkg_source=http://tukaani.org/${pkg_distname}/${pkg_distname}-${pkg_version}.tar.gz
pkg_shasum=73df4d5d34f0468bd57d09f2d8af363e95ed6cc3a4a86129d2f2c366259902a2
pkg_dirname=${pkg_distname}-${pkg_version}
pkg_deps=(core/glibc)
pkg_build_deps=(core/coreutils core/diffutils core/patch core/make core/gcc core/sed)
pkg_bin_dirs=(bin)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)

do_check() {
  make check
}


# ----------------------------------------------------------------------------
# **NOTICE:** What follows are implementation details required for building a
# first-pass, "stage1" toolchain and environment. It is only used when running
# in a "stage1" Studio and can be safely ignored by almost everyone. Having
# said that, it performs a vital bootstrapping process and cannot be removed or
# significantly altered. Thank you!
# ----------------------------------------------------------------------------
if [[ "$STUDIO_TYPE" = "stage1" ]]; then
  pkg_build_deps=(core/gcc core/coreutils core/sed core/diffutils)
fi
