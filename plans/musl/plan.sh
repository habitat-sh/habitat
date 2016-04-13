pkg_name=musl
pkg_origin=chef
pkg_version=1.1.12
pkg_license=('mit')
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_source=http://www.musl-libc.org/releases/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=720b83c7e276b4b679c0bffe9509340d5f81fd601508e607e708177df0d31c0e
pkg_deps=()
pkg_build_deps=(chef/coreutils chef/diffutils chef/patch chef/make chef/gcc chef/sed)
pkg_bin_dirs=(bin)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)

do_build() {
  ./configure \
    --prefix=$pkg_prefix \
    --syslibdir=$pkg_prefix/lib
  make -j$(nproc)
}

do_install() {
  do_default_install

  # Install license
  install -Dm0644 COPYRIGHT $pkg_prefix/share/licenses/COPYRIGHT
}


# ----------------------------------------------------------------------------
# **NOTICE:** What follows are implementation details required for building a
# first-pass, "stage1" toolchain and environment. It is only used when running
# in a "stage1" Studio and can be safely ignored by almost everyone. Having
# said that, it performs a vital bootstrapping process and cannot be removed or
# significantly altered. Thank you!
# ----------------------------------------------------------------------------
if [[ "$STUDIO_TYPE" = "stage1" ]]; then
  pkg_build_deps=(chef/gcc chef/coreutils chef/sed chef/diffutils chef/make chef/patch)
fi
