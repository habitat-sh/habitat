pkg_name=zlib
pkg_derivation=chef
pkg_version=1.2.8
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_license=('zlib')
pkg_source=http://zlib.net/current/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=36658cb768a54c1d4dec43c3116c27ed893e88b02ecfcb44f2166f9c0b7f2a0d
pkg_deps=(chef/glibc)
pkg_lib_dirs=(lib)
pkg_include_dirs=(include)
pkg_gpg_key=3853DA6B

do_prepare() {
  # TODO: We need a more clever way to calculate/determine the path to ld-*.so
  LDFLAGS="$LDFLAGS -Wl,-rpath=${LD_RUN_PATH},--enable-new-dtags"
  LDFLAGS="$LDFLAGS -Wl,--dynamic-linker=$(pkg_path_for glibc)/lib/ld-2.22.so"
  export LDFLAGS
  build_line "Updating LDFLAGS=$LDFLAGS"
}
