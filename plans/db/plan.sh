pkg_name=db
pkg_derivation=chef
pkg_version=5.3.28
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_license=('custom')
pkg_source=http://download.oracle.com/berkeley-db/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=e0a992d740709892e81f9d93f06daf305cf73fb81b545afe72478043172c3628
pkg_deps=(chef/glibc chef/gcc-libs)
pkg_build_deps=(chef/gcc chef/coreutils chef/sed chef/bison chef/flex chef/grep chef/bash chef/gawk chef/libtool)
pkg_binary_path=(bin)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)
pkg_gpg_key=3853DA6B

do_build() {
  pushd build_unix > /dev/null
  ../dist/configure \
    --prefix=$pkg_prefix \
    --enable-compat185 \
    --enable-cxx \
    --enable-dbm \
    --enable-stl
  make LIBSO_LIBS=-lpthread -j$(nproc)
  popd > /dev/null
}

do_install() {
  pushd build_unix > /dev/null
  do_default_install
  popd > /dev/null

  # Install license file
  install -Dm644 LICENSE "$pkg_path/share/licenses/LICENSE"
}
