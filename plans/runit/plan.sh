pkg_name=runit
pkg_origin=chef
pkg_version=2.1.2
pkg_license=('BSD')
pkg_source=http://smarden.org/runit/runit-2.1.2.tar.gz
pkg_shasum=6fd0160cb0cf1207de4e66754b6d39750cff14bb0aa66ab49490992c0c47ba18
pkg_build_deps=(chef/coreutils chef/gcc chef/make)
pkg_deps=(chef/glibc)
pkg_bin_dirs=(bin)

do_unpack() {
  mkdir -p $HAB_CACHE_SRC_PATH/${pkg_name}-${pkg_version}
  tar zxf $HAB_CACHE_SRC_PATH/$pkg_filename -C $HAB_CACHE_SRC_PATH/${pkg_name}-${pkg_version}
}

do_build() {
  pushd admin/runit-${pkg_version}
  ./package/compile
  popd
}

do_install() {
  mkdir -p $pkg_prefix/bin
  cp admin/runit-${pkg_version}/command/* $pkg_prefix/bin
}
