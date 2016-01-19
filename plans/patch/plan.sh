pkg_name=patch
pkg_derivation=chef
pkg_version=2.7.5
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_license=('gplv3+')
pkg_source=http://ftp.gnu.org/gnu/$pkg_name/${pkg_name}-${pkg_version}.tar.xz
pkg_shasum=fd95153655d6b95567e623843a0e77b81612d502ecf78a489a4aed7867caa299
pkg_deps=(chef/glibc chef/attr)
pkg_build_deps=(chef/gcc chef/coreutils chef/sed chef/bison chef/flex chef/grep chef/bash chef/gawk chef/libtool chef/diffutils chef/findutils chef/xz chef/gettext chef/gzip chef/make)
pkg_binary_path=(bin)
pkg_gpg_key=3853DA6B

do_check() {
  make check
}
