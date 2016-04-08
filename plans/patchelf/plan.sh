pkg_name=patchelf
pkg_origin=chef
pkg_version=0.8
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('gplv3')
pkg_source=http://releases.nixos.org/$pkg_name/${pkg_name}-$pkg_version/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=14af06a2da688d577d64ff8dac065bb8903bbffbe01d30c62df7af9bf4ce72fe
pkg_build_deps=(chef/coreutils chef/diffutils chef/patch chef/make chef/gcc)
pkg_bin_dirs=(bin)
pkg_gpg_key=3853DA6B

if [[ -n "$FIRST_PASS" ]]; then
  # Waiting on chef/gcc-libs to link libgcc and libstdc++, but because we need
  # this package to prepare chef/gcc-libs, we'll do the cheap version first
  # that relies on the full gcc version of these shared libraries
  pkg_deps=(chef/glibc chef/gcc)
  build_line "Using libgcc and libstdc++ from chef/gcc"
else
  pkg_deps=(chef/glibc chef/gcc-libs)
fi


# ----------------------------------------------------------------------------
# **NOTICE:** What follows are implementation details required for building a
# first-pass, "stage1" toolchain and environment. It is only used when running
# in a "stage1" Studio and can be safely ignored by almost everyone. Having
# said that, it performs a vital bootstrapping process and cannot be removed or
# significantly altered. Thank you!
# ----------------------------------------------------------------------------
if [[ "$STUDIO_TYPE" = "stage1" ]]; then
  pkg_build_deps=(chef/gcc)
fi
