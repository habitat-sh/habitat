pkg_name=mg
pkg_origin=chef
pkg_version=20160118
pkg_license=('publicdomain')
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_source=http://homepage.boetes.org/software/$pkg_name/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=26450b2564bec0b0afc465fd24a1917dc31508c5500c3a36823b9c763a2b8636
pkg_deps=(chef/glibc chef/ncurses chef/libbsd)
pkg_build_deps=(chef/coreutils chef/diffutils chef/patch chef/make chef/gcc chef/sed chef/pkg-config chef/clens)
pkg_binary_path=(bin)
pkg_gpg_key=3853DA6B

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
  install -Dm644 README "$pkg_path/share/licenses/README"
}


# ----------------------------------------------------------------------------
# **NOTICE:** What follows are implementation details required for building a
# first-pass, "stage1" toolchain and environment. It is only used when running
# in a "stage1" Studio and can be safely ignored by almost everyone. Having
# said that, it performs a vital bootstrapping process and cannot be removed or
# significantly altered. Thank you!
# ----------------------------------------------------------------------------
if [[ "$STUDIO_TYPE" = "stage1" ]]; then
  pkg_build_deps=(chef/gcc chef/pkg-config chef/coreutils chef/sed chef/diffutils chef/make chef/patch chef/clens)
fi
