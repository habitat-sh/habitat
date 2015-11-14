pkg_name=runit
pkg_derivation=chef
pkg_version=2.1.2
pkg_license=('BSD')
pkg_source=http://smarden.org/runit/runit-2.1.2.tar.gz
pkg_shasum=6fd0160cb0cf1207de4e66754b6d39750cff14bb0aa66ab49490992c0c47ba18
pkg_gpg_key=3853DA6B
pkg_deps=(chef/glibc)
pkg_binary_path=(bin)

unpack() {
  mkdir -p $BLDR_SRC_CACHE/${pkg_name}-${pkg_version}
  tar zxf $BLDR_SRC_CACHE/$pkg_filename -C $BLDR_SRC_CACHE/${pkg_name}-${pkg_version}
}

build() {
  pushd admin/runit-${pkg_version}
  ./package/compile
  popd
}

install() {
  mkdir -p $pkg_prefix/bin
  cp admin/runit-${pkg_version}/command/* $pkg_prefix/bin
}
