pkg_name=expect
pkg_origin=core
pkg_version=5.45
pkg_license=('custom')
pkg_maintainer="The Habitat Contributors <humans@habitat.sh>"
pkg_source=http://downloads.sourceforge.net/project/$pkg_name/Expect/${pkg_version}/${pkg_name}${pkg_version}.tar.gz
pkg_shasum=b28dca90428a3b30e650525cdc16255d76bb6ccd65d448be53e620d95d5cc040
pkg_dirname=${pkg_name}${pkg_version}
pkg_deps=(core/glibc core/tcl core/coreutils)
pkg_build_deps=(core/coreutils core/diffutils core/patch core/make core/gcc)
pkg_bin_dirs=(bin)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)

do_prepare() {
  do_default_prepare

  # Make the path to `stty` absolute from `coreutils`
  sed -i "s,/bin/stty,$(pkg_path_for coreutils)/bin/stty,g" configure
}

do_build() {
  ./configure \
    --prefix=$pkg_prefix \
    --exec-prefix=$pkg_prefix \
    --with-tcl=$(pkg_path_for tcl)/lib \
    --with-tclinclude=$(pkg_path_for tcl)/include
  make
}

do_check() {
  # The test suite looks for `libgcc_s`, so we'll add it to the
  # `LD_LIBRARY_PATH`.
  make test LD_LIBRARY_PATH="$(pkg_path_for gcc)/lib"
}

do_install() {
  make install LD_LIBRARY_PATH="$(pkg_path_for gcc)/lib"

  # Add an absolute path to `tclsh` in each script binary
  find $pkg_prefix/bin \
    -type f \
    -exec sed -e "s,exec tclsh,exec $(pkg_path_for tcl)/bin/tclsh,g" -i {} \;
}


# ----------------------------------------------------------------------------
# **NOTICE:** What follows are implementation details required for building a
# first-pass, "stage1" toolchain and environment. It is only used when running
# in a "stage1" Studio and can be safely ignored by almost everyone. Having
# said that, it performs a vital bootstrapping process and cannot be removed or
# significantly altered. Thank you!
# ----------------------------------------------------------------------------
if [[ "$STUDIO_TYPE" = "stage1" ]]; then
  pkg_build_deps=(core/gcc core/coreutils core/diffutils core/make core/patch)
fi
