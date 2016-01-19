pkg_name=m4
pkg_derivation=chef
pkg_version=1.4.17
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_license=('gplv3')
pkg_source=http://ftp.gnu.org/gnu/$pkg_name/${pkg_name}-${pkg_version}.tar.xz
pkg_shasum=f0543c3beb51fa6b3337d8025331591e0e18d8ec2886ed391f1aade43477d508
pkg_build_deps=(chef/binutils)
pkg_deps=(chef/glibc)
pkg_binary_path=(bin)
pkg_gpg_key=3853DA6B

do_prepare() {
  # Force gcc to use our ld wrapper from binutils when calling `ld`
  CFLAGS="$CFLAGS -B$(pkg_path_for binutils)/bin/"
  build_line "Updating CFLAGS=$CFLAGS"
}

do_check() {
  # Fixes a broken test with either gcc 5.2.x and/or perl 5.22.x:
  # FAIL: test-update-copyright.sh
  #
  # Thanks to: http://permalink.gmane.org/gmane.linux.lfs.devel/16285
  sed -i 's/copyright{/copyright\\{/' build-aux/update-copyright

  make check
}
