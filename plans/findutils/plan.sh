pkg_name=findutils
pkg_origin=core
pkg_version=4.4.2
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('gplv3+')
pkg_source=http://ftp.gnu.org/gnu/$pkg_name/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=434f32d171cbc0a5e72cfc5372c6fc4cb0e681f8dce566a0de5b6fccd702b62a
pkg_deps=(core/glibc)
pkg_build_deps=(core/coreutils core/diffutils core/patch core/make core/gcc core/sed)
pkg_bin_dirs=(bin)

do_prepare() {
  do_default_prepare

  # Fix a few hardcoded, absolute paths in the codebase.
  #
  # Thanks to: https://github.com/NixOS/nixpkgs/blob/release-15.09/pkgs/tools/misc/findutils/default.nix#L28
  patch -p1 -i $PLAN_CONTEXT/updatedb-path.patch
  patch -p1 -i $PLAN_CONTEXT/xargs-echo-path.patch
  patch -p1 -i $PLAN_CONTEXT/disable-test-canonicalize.patch
}

do_build() {
  ./configure \
    --prefix=$pkg_prefix \
    --localstatedir=$pkg_svc_var_path/locate
  make
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
