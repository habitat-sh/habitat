pkg_name=xz
pkg_derivation=chef
pkg_version=5.2.2
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_license=('gpl2+' 'lgpl2+')
pkg_source=http://tukaani.org/${pkg_name}/${pkg_name}-${pkg_version}.tar.gz
pkg_filename=${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=73df4d5d34f0468bd57d09f2d8af363e95ed6cc3a4a86129d2f2c366259902a2
pkg_deps=(chef/glibc)
pkg_build_deps=(chef/gcc chef/coreutils chef/sed chef/bison chef/flex chef/grep chef/bash chef/gawk chef/libtool chef/diffutils)
pkg_binary_path=(bin)
pkg_lib_dirs=(lib)
pkg_include_dirs=(include)
pkg_gpg_key=3853DA6B

do_build() {
  do_default_build

  if [[ -n "$DO_CHECK" ]]; then
    build_line "Running post-compile tests"
    make check
  fi
}
