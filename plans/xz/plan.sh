pkg_name=xz
pkg_derivation=chef
pkg_version=5.2.2
pkg_license=('LGPL')
pkg_source=http://tukaani.org/${pkg_name}/${pkg_name}-${pkg_version}.tar.gz
pkg_filename=${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=73df4d5d34f0468bd57d09f2d8af363e95ed6cc3a4a86129d2f2c366259902a2
pkg_gpg_key=3853DA6B
pkg_deps=()
pkg_lib_dirs=(lib)
pkg_include_dirs=(include)

do_build() {
  ./configure \
    --prefix=$pkg_prefix
  make
}
