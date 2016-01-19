pkg_name=gettext
pkg_derivation=chef
pkg_version=0.19.6
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_license=('gplv2+' 'lgpl2+')
pkg_source=http://ftp.gnu.org/gnu/$pkg_name/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=ed4b4c19bd3a3034eb6769500a3592ff616759ef43cf30586dbb7a17c9dd695d
pkg_deps=(chef/glibc chef/gcc-libs chef/acl chef/xz)
pkg_build_deps=(chef/gcc chef/coreutils chef/sed chef/bison chef/flex chef/grep chef/bash chef/gawk chef/libtool chef/diffutils chef/findutils)
pkg_binary_path=(bin)
pkg_lib_dirs=(lib)
pkg_include_dirs=(include)
pkg_gpg_key=3853DA6B

do_build() {
  ./configure \
    --prefix=$pkg_prefix
  make -j$(nproc)

  if [[ -n "$DO_CHECK" ]]; then
    build_line "Running post-compile tests"
    make check
  fi
}
