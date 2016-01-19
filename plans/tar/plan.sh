pkg_name=tar
pkg_derivation=chef
pkg_version=1.28
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_license=('gplv3+')
pkg_source=http://ftp.gnu.org/gnu/$pkg_name/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=6a6b65bac00a127a508533c604d5bf1a3d40f82707d56f20cefd38a05e8237de
pkg_deps=(chef/glibc chef/acl chef/attr)
pkg_build_deps=(chef/gcc chef/coreutils chef/sed chef/bison chef/flex chef/grep chef/bash)
pkg_binary_path=(bin)
pkg_gpg_key=3853DA6B

do_build() {
  # * `FORCE_UNSAFE_CONFIGURE` forces the test for `mknod` to be run as root
  FORCE_UNSAFE_CONFIGURE=1 ./configure \
    --prefix=$pkg_prefix
  make
}

do_check() {
  make check
}
