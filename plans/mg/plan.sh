pkg_name=mg
pkg_origin=core
pkg_version=20160118
pkg_license=('publicdomain')
pkg_maintainer="The Habitat Contributors <humans@habitat.sh>"
pkg_source=http://homepage.boetes.org/software/$pkg_name/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=26450b2564bec0b0afc465fd24a1917dc31508c5500c3a36823b9c763a2b8636
pkg_deps=(core/glibc core/ncurses core/libbsd)
pkg_build_deps=(core/coreutils core/diffutils core/patch core/make core/gcc core/sed core/pkg-config core/clens)
pkg_bin_dirs=(bin)

do_prepare() {
  cat $PLAN_CONTEXT/cleanup.patch \
    | sed \
      -e "s,@prefix@,$pkg_prefix,g" \
      -e "s,@clens_prefix@,$(pkg_path_for clens),g" \
      -e "s,@libbsd_prefix@,$(pkg_path_for libbsd),g" \
    | patch -p1

  export PKG_CONFIG_PATH=$(pkg_path_for libbsd)/lib/pkgconfig
}
do_build() {
  make \
    prefix=$pkg_prefix \
    PKG_CONFIG=pkg-config \
    INSTALL=install \
    STRIP=strip
}

do_install() {
  do_default_install

  # Install license file from README
  install -Dm644 README "$pkg_prefix/share/licenses/README"
}


# ----------------------------------------------------------------------------
# **NOTICE:** What follows are implementation details required for building a
# first-pass, "stage1" toolchain and environment. It is only used when running
# in a "stage1" Studio and can be safely ignored by almost everyone. Having
# said that, it performs a vital bootstrapping process and cannot be removed or
# significantly altered. Thank you!
# ----------------------------------------------------------------------------
if [[ "$STUDIO_TYPE" = "stage1" ]]; then
  pkg_build_deps=(core/gcc core/pkg-config core/coreutils core/sed core/diffutils core/make core/patch core/clens)
fi
