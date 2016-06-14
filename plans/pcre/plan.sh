pkg_name=pcre
pkg_origin=core
pkg_version=8.38
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('bsd')
pkg_source=http://ftp.csx.cam.ac.uk/pub/software/programming/${pkg_name}/${pkg_name}-${pkg_version}.tar.bz2
pkg_shasum=b9e02d36e23024d6c02a2e5b25204b3a4fa6ade43e0a5f869f254f49535079df
pkg_deps=(core/glibc core/gcc-libs)
pkg_build_deps=(core/coreutils core/diffutils core/patch core/make core/gcc)
pkg_bin_dirs=(bin)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)

do_build() {
  ./configure \
    --prefix=$pkg_prefix \
    --enable-unicode-properties \
    --enable-pcre16 \
    --enable-pcre32 \
    --enable-jit
  make -j$(nproc)
}

do_check() {
  make check
}

do_install() {
  do_default_install

  # Install license file
  install -Dm644 LICENCE $pkg_prefix/share/licenses/LICENSE
}


# ----------------------------------------------------------------------------
# **NOTICE:** What follows are implementation details required for building a
# first-pass, "stage1" toolchain and environment. It is only used when running
# in a "stage1" Studio and can be safely ignored by almost everyone. Having
# said that, it performs a vital bootstrapping process and cannot be removed or
# significantly altered. Thank you!
# ----------------------------------------------------------------------------
if [[ "$STUDIO_TYPE" = "stage1" ]]; then
  pkg_build_deps=(core/gcc core/coreutils)
fi
