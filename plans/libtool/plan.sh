pkg_name=libtool
pkg_origin=core
pkg_version=2.4.6
pkg_license=('gplv2+')
pkg_maintainer="The Habitat Contributors <humans@habitat.sh>"
pkg_source=http://ftp.gnu.org/gnu/${pkg_name}/${pkg_name}-${pkg_version}.tar.gz
pkg_filename=${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=e3bd4d5d3d025a36c21dd6af7ea818a2afcd4dfc1ea5a17b39d7854bcd0c06e3
pkg_deps=(core/glibc core/coreutils core/sed core/grep)
pkg_build_deps=(core/coreutils core/diffutils core/patch core/make core/gcc core/m4)
pkg_bin_dirs=(bin)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)

do_prepare() {
  do_default_prepare

  # Drop the dependency on `help2man` by skipping the generation of a man page
  sed \
    -e '/^dist_man1_MANS =/ s,^.*$,dist_man1_MANS = $(libtoolize_1),g' \
    -i Makefile.in
}

do_build() {
  # * `lt_cv_sys_dlsearch_path` Makes the default library search path empty,
  # rather than `"/lib /usr/lib"`
  ./configure \
    --prefix=$pkg_prefix \
    lt_cv_sys_lib_dlsearch_path_spec="" \
    lt_cv_sys_lib_search_path_spec=""
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
  pkg_build_deps=(core/gcc core/m4)
fi
