pkg_name=vim
pkg_origin=core
pkg_version=7.4.1089
pkg_license=('vim')
pkg_maintainer="The Habitat Contributors <humans@habitat.sh>"
pkg_source=http://github.com/$pkg_name/$pkg_name/archive/v${pkg_version}.tar.gz
pkg_shasum=e52f7653a36b690441b47a273b1db72f0eb1e5f6729af25110a84088ca73e872
pkg_deps=(core/glibc core/acl core/ncurses)
pkg_build_deps=(core/coreutils core/diffutils core/patch core/make core/gcc core/sed core/autoconf)
pkg_bin_dirs=(bin)

do_prepare() {
  pushd src > /dev/null
    autoconf
  popd > /dev/null

  export CPPFLAGS="$CPPFLAGS $CFLAGS"
  build_line "Setting CPPFLAGS=$CPPFLAGS"
}

do_build() {
  ./configure \
    --prefix=$pkg_prefix \
    --with-compiledby="Habitat, vim release $pkg_version" \
    --with-features=huge \
    --enable-acl \
    --with-x=no \
    --disable-gui \
    --enable-multibyte
  make
}

do_install() {
  do_default_install

  # Add a `vi` which symlinks to `vim`
  ln -sv vim $pkg_prefix/bin/vi

  # Install license file
  install -Dm644 runtime/doc/uganda.txt $pkg_prefix/share/licenses/license.txt
}


# ----------------------------------------------------------------------------
# **NOTICE:** What follows are implementation details required for building a
# first-pass, "stage1" toolchain and environment. It is only used when running
# in a "stage1" Studio and can be safely ignored by almost everyone. Having
# said that, it performs a vital bootstrapping process and cannot be removed or
# significantly altered. Thank you!
# ----------------------------------------------------------------------------
if [[ "$STUDIO_TYPE" = "stage1" ]]; then
  pkg_build_deps=(core/gcc core/coreutils core/sed core/diffutils core/make core/patch core/autoconf)
fi
