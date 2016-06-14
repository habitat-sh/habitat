pkg_name=make
pkg_origin=core
pkg_version=4.1
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('gplv3+')
pkg_source=http://ftp.gnu.org/gnu/$pkg_name/${pkg_name}-${pkg_version}.tar.bz2
pkg_shasum=0bc7613389650ee6a24554b52572a272f7356164fd2c4132b0bcf13123e4fca5
pkg_deps=(core/glibc)
pkg_build_deps=(core/coreutils core/diffutils core/patch core/make core/gcc core/bash core/gettext core/gzip core/perl core/binutils)
pkg_bin_dirs=(bin)
pkg_include_dirs=(include)

do_prepare() {
  do_default_prepare

  # Don't look for library dependencies in the root system (i.e. `/lib`,
  # `/usr/lib`, etc.)
  patch -p1 -i $PLAN_CONTEXT/no-sys-dirs.patch
}

do_check() {
  # Force `ar` to not run in deterministic mode, as the testsuite relies on
  # UID, GID, timestamp and file mode values to be correctly stored.
  #
  # Thanks to: https://bugs.debian.org/cgi-bin/bugreport.cgi?bug=782750
  mkdir -pv wrappers
  cat <<EOF > wrappers/ar
#!$(pkg_path_for bash)/bin/sh
exec $(pkg_path_for binutils)/bin/ar U\$@
EOF
  chmod -v 0744 wrappers/ar

  env PATH="$(pwd)/wrappers:$PATH" make check
}


# ----------------------------------------------------------------------------
# **NOTICE:** What follows are implementation details required for building a
# first-pass, "stage1" toolchain and environment. It is only used when running
# in a "stage1" Studio and can be safely ignored by almost everyone. Having
# said that, it performs a vital bootstrapping process and cannot be removed or
# significantly altered. Thank you!
# ----------------------------------------------------------------------------
if [[ "$STUDIO_TYPE" = "stage1" ]]; then
  pkg_build_deps=(core/binutils core/gcc core/coreutils core/sed core/bash core/perl core/diffutils core/gettext core/gzip)
fi
