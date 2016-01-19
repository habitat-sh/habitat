pkg_name=file
pkg_derivation=chef
pkg_version=5.24
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_license=('custom')
pkg_source=ftp://ftp.astron.com/pub/$pkg_name/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=802cb3de2e49e88ef97cdcb52cd507a0f25458112752e398445cea102bc750ce
pkg_deps=(chef/glibc chef/zlib)
pkg_binary_path=(bin)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)
pkg_gpg_key=3853DA6B

do_prepare() {
  do_default_prepare

  # TODO: We need a more clever way to calculate/determine the path to ld-*.so
  LDFLAGS="$LDFLAGS -Wl,-rpath=${LD_RUN_PATH},--enable-new-dtags"
  LDFLAGS="$LDFLAGS -Wl,--dynamic-linker=$(pkg_path_for glibc)/lib/ld-2.22.so"
  export LDFLAGS
  build_line "Updating LDFLAGS=$LDFLAGS"
}

do_check() {
  make check
}

do_install() {
  make install

  # As per the copyright, we must include the copyright statement in binary
  # distributions
  #
  # Source: https://github.com/file/file/blob/master/COPYING
  install -v -Dm644 COPYING "$pkg_path/share/COPYING"
}
