pkg_name=pkg-config
pkg_origin=chef
pkg_version=0.29
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('gplv2+')
pkg_source=http://pkgconfig.freedesktop.org/releases/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=c8507705d2a10c67f385d66ca2aae31e81770cc0734b4191eb8c489e864a006b
pkg_deps=(chef/glibc)
pkg_build_deps=(chef/coreutils chef/diffutils chef/patch chef/make chef/gcc)
pkg_bin_dirs=(bin)
pkg_gpg_key=3853DA6B

do_build() {
  ./configure \
    --prefix=$pkg_prefix \
    --with-internal-glib \
    --disable-host-tool
  make
}

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
  pkg_build_deps=(chef/gcc)
fi
