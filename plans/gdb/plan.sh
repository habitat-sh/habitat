pkg_name=gdb
pkg_origin=core
pkg_version=7.11
pkg_maintainer="The Habitat Contributors <humans@habitat.sh>"
pkg_license=('gplv3')
pkg_source=http://ftp.gnu.org/gnu/$pkg_name/${pkg_name}-${pkg_version}.tar.xz
pkg_shasum=7a434116cb630d77bb40776e8f5d3937bed11dea56bafebb4d2bc5dd389fe5c1
pkg_deps=(core/glibc core/readline core/zlib core/python)
pkg_build_deps=(core/coreutils core/diffutils core/patch core/make core/gcc core/texinfo)
pkg_bin_dirs=(bin)
pkg_lib_dirs=(lib)

do_build() {
  ./configure \
    --prefix=$pkg_prefix \
    --with-system-readline \
    --with-system-zlib
  make
}

do_check() {
  make check
}

do_install() {
  do_default_install

  # Clean up files that ship with binutils and may conflict
  rm -fv $pkg_prefix/lib/{libbfd,libopcodes}.a
  rm -fv $pkg_prefix/include/{ansidecl,bfd,bfdlink,dis-asm,plugin-api,symcat}.h
  rm -fv $pkg_prefix/share/info/bfd.info
}
