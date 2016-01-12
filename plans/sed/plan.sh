pkg_name=sed
pkg_derivation=chef
pkg_version=4.2.2
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_license=('gplv3')
pkg_source=http://ftp.gnu.org/gnu/$pkg_name/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=fea0a94d4b605894f3e2d5572e3f96e4413bcad3a085aae7367c2cf07908b2ff
pkg_deps=(chef/glibc chef/acl)
pkg_build_deps=(chef/binutils chef/gcc)
pkg_binary_path=(bin)
pkg_gpg_key=3853DA6B

do_build() {
  do_default_build

  if [[ -n "$DO_CHECK" ]]; then
    make check
  fi
}
