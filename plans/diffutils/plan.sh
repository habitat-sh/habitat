pkg_name=diffutils
pkg_derivation=chef
pkg_version=3.3
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_license=('gplv3+')
pkg_source=http://ftp.gnu.org/gnu/$pkg_name/${pkg_name}-${pkg_version}.tar.xz
pkg_shasum=a25e89a8ab65fded1731e4186be1bb25cda967834b6df973599cdcd5abdfc19c
pkg_deps=(chef/glibc)
pkg_build_deps=(chef/gcc chef/coreutils chef/sed chef/bison chef/flex chef/grep chef/bash chef/gawk chef/libtool)
pkg_binary_path=(bin)
pkg_gpg_key=3853DA6B

do_build() {
  do_default_build

  if [[ -n "$DO_CHECK" ]]; then
    build_line "Running post-compile tests"

    # Fixes a broken test with either gcc 5.2.x and/or perl 5.22.x:
    # FAIL: test-update-copyright.sh
    #
    # Thanks to: http://permalink.gmane.org/gmane.linux.lfs.devel/16285
    sed -i 's/copyright{/copyright\\{/' build-aux/update-copyright

    make check
  fi
}
