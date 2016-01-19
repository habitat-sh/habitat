pkg_name=findutils
pkg_derivation=chef
pkg_version=4.4.2
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_license=('gplv3+')
pkg_source=http://ftp.gnu.org/gnu/$pkg_name/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=434f32d171cbc0a5e72cfc5372c6fc4cb0e681f8dce566a0de5b6fccd702b62a
pkg_deps=(chef/glibc)
pkg_build_deps=(chef/gcc chef/coreutils chef/sed chef/bison chef/flex chef/grep chef/bash chef/gawk chef/libtool chef/diffutils)
pkg_binary_path=(bin)
pkg_gpg_key=3853DA6B

do_prepare() {
  do_default_prepare

  # Fix a few hardcoded, absolute paths in the codebase.
  #
  # Thanks to: https://github.com/NixOS/nixpkgs/blob/release-15.09/pkgs/tools/misc/findutils/default.nix#L28
  patch -p1 -i $PLAN_CONTEXT/updatedb-path.patch
  patch -p1 -i $PLAN_CONTEXT/xargs-echo-path.patch
  patch -p1 -i $PLAN_CONTEXT/disable-test-canonicalize.patch
}

do_build() {
  ./configure \
    --prefix=$pkg_prefix \
    --localstatedir=$pkg_srvc_var/locate
  make
}
