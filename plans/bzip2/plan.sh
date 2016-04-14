pkg_name=bzip2
pkg_distname=$pkg_name
pkg_origin=chef
pkg_version=1.0.6
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('bzip2')
pkg_source=http://www.bzip.org/$pkg_version/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=a2848f34fcd5d6cf47def00461fcb528a0484d8edef8208d6d2e2909dc61d9cd
pkg_dirname=${pkg_distname}-${pkg_version}
pkg_deps=(chef/glibc)
pkg_build_deps=(chef/coreutils chef/diffutils chef/patch chef/make chef/gcc)
pkg_bin_dirs=(bin)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)

_common_prepare() {
  # Makes the symbolic links in installation relative vs. absolute
  sed -i 's@\(ln -s -f \)$(PREFIX)/bin/@\1@' Makefile

  # Ensure that the man pages are installed under share/man
  sed -i "s@(PREFIX)/man@(PREFIX)/share/man@g" Makefile
}

do_prepare() {
  _common_prepare

  export CC=gcc
  build_line "Setting CC=$CC"
}

do_build() {
  make -f Makefile-libbz2_so PREFIX="$pkg_prefix" CC="$CC"
  make bzip2 bzip2recover CC="$CC" LDFLAGS="$LDFLAGS"
}

do_check() {
  make test
}

do_install() {
  local maj=$(echo $pkg_version | cut -d "." -f 1)
  local maj_min=$(echo $pkg_version | cut -d "." -f 1-2)

  make install PREFIX="$pkg_prefix"

  # Replace some hard links with symlinks
  rm -fv $pkg_prefix/bin/{bunzip2,bzcat}
  ln -sv bzip2 $pkg_prefix/bin/bunzip2
  ln -sv bzip2 $pkg_prefix/bin/bzcat

  # Install the shared library and its symlinks
  cp -v $HAB_CACHE_SRC_PATH/$pkg_dirname/libbz2.so.$pkg_version $pkg_prefix/lib
  ln -sv libbz2.so.$pkg_version $pkg_prefix/lib/libbz2.so
  ln -sv libbz2.so.$pkg_version $pkg_prefix/lib/libbz2.so.$maj
  ln -sv libbz2.so.$pkg_version $pkg_prefix/lib/libbz2.so.$maj_min
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
