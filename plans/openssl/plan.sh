pkg_name=openssl
pkg_distname=$pkg_name
pkg_origin=core
pkg_version=1.0.2g
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('bsd')
pkg_source=https://www.openssl.org/source/${pkg_distname}-${pkg_version}.tar.gz
pkg_shasum=b784b1b3907ce39abf4098702dade6365522a253ad1552e267a9a0e89594aa33
pkg_dirname=${pkg_distname}-${pkg_version}
pkg_deps=(core/glibc core/zlib core/cacerts)
pkg_build_deps=(core/coreutils core/diffutils core/patch core/make core/gcc core/sed core/grep core/perl)
pkg_bin_dirs=(bin)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)

_common_prepare() {
  do_default_prepare

  # Set CA dir to `$pkg_prefix/ssl` by default and use the cacerts from the
  # `cacerts` package. Note that `patch(1)` is making backups because
  # we need an original for the test suite.
  cat $PLAN_CONTEXT/ca-dir.patch \
    | sed \
      -e "s,@prefix@,$pkg_prefix,g" \
      -e "s,@cacerts_prefix@,$(pkg_path_for cacerts),g" \
    | patch -p1 --backup

  # Purge the codebase (mostly tests) of the hardcoded reliance on `/bin/rm`.
  grep -lr '/bin/rm' . | while read f; do
    sed -e 's,/bin/rm,rm,g' -i "$f"
  done
}

do_prepare() {
  export BUILD_CC=gcc
  build_line "Setting BUILD_CC=$BUILD_CC"
}

do_build() {
  ./config \
    --prefix=${pkg_prefix} \
    --openssldir=ssl \
    no-idea \
    no-mdc2 \
    no-rc5 \
    zlib \
    shared \
    disable-gost \
    $CFLAGS \
    $LDFLAGS
  env CC= make depend
  make CC="$BUILD_CC"
}

do_check() {
  # Flip back to the original sources to satisfy the test suite, but keep the
  # final version for packaging.
  for f in apps/CA.pl.in apps/CA.sh apps/openssl.cnf; do
    cp -fv $f ${f}.final
    cp -fv ${f}.orig $f
  done

  make test

  # Finally, restore the final sources to their original locations.
  for f in apps/CA.pl.in apps/CA.sh apps/openssl.cnf; do
    cp -fv ${f}.final $f
  done
}

do_install() {
  do_default_install

  # Remove dependency on Perl at runtime
  rm -rfv $pkg_prefix/ssl/misc $pkg_prefix/bin/c_rehash
}


# ----------------------------------------------------------------------------
# **NOTICE:** What follows are implementation details required for building a
# first-pass, "stage1" toolchain and environment. It is only used when running
# in a "stage1" Studio and can be safely ignored by almost everyone. Having
# said that, it performs a vital bootstrapping process and cannot be removed or
# significantly altered. Thank you!
# ----------------------------------------------------------------------------
if [[ "$STUDIO_TYPE" = "stage1" ]]; then
  pkg_build_deps=(core/gcc core/coreutils core/sed core/grep core/perl core/diffutils core/make core/patch)
fi
