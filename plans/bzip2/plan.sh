pkg_name=bzip2
pkg_derivation=chef
pkg_version=1.0.6
pkg_license=('bzip2')
pkg_source=http://www.bzip.org/1.0.6/bzip2-1.0.6.tar.gz
pkg_shasum=a2848f34fcd5d6cf47def00461fcb528a0484d8edef8208d6d2e2909dc61d9cd
pkg_gpg_key=3853DA6B
pkg_deps=(chef/glibc)
pkg_lib_dirs=(lib)
pkg_include_dirs=(include)
pkg_binary_path=(bin)

do_build() {
  make -f Makefile-libbz2_so PREFIX="$pkg_prefix" LDFLAGS="$LDFLAGS"
  make bzip2 bzip2recover
}

do_install() {
  make install PREFIX="$pkg_prefix"
  cp $BLDR_SRC_CACHE/$pkg_dirname/libbz2.so.1.0.6 $pkg_prefix/lib
  ln -s $pkg_prefix/lib/libbz2.so.1.0.6 $pkg_prefix/lib/libbz2.so
  ln -s $pkg_prefix/lib/libbz2.so.1.0.6 $pkg_prefix/lib/libbz2.so.1
  ln -s $pkg_prefix/lib/libbz2.so.1.0.6 $pkg_prefix/lib/libbz2.so.1.0
}

