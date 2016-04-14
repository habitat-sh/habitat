pkg_name=zeromq
pkg_origin=core
pkg_version=4.1.4
pkg_license=('LGPL')
pkg_source=http://download.zeromq.org/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=e99f44fde25c2e4cb84ce440f87ca7d3fe3271c2b8cfbc67d55e4de25e6fe378
pkg_deps=(chef/glibc chef/gcc-libs chef/libsodium)
pkg_build_deps=(chef/gcc chef/coreutils chef/make chef/pkg-config chef/patchelf)
pkg_lib_dirs=(lib)
pkg_include_dirs=(include)

do_prepare() {
  export PKG_CONFIG_PATH=$(pkg_path_for chef/libsodium)/lib/pkgconfig
}

do_install() {
  do_default_install
  find $pkg_prefix/lib -name *.so | xargs -I '%' patchelf --set-rpath "$LD_RUN_PATH" %
}
