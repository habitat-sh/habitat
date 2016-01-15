pkg_name=grep
pkg_derivation=chef
pkg_version=2.22
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_license=('gplv3+')
pkg_source=http://ftp.gnu.org/gnu/$pkg_name/${pkg_name}-${pkg_version}.tar.xz
pkg_shasum=ca91d22f017bfcb503d4bc3b44295491c89a33a3df0c3d8b8614f2d3831836eb
pkg_deps=(chef/glibc chef/pcre)
pkg_build_deps=(chef/gcc chef/coreutils)
pkg_binary_path=(bin)
pkg_gpg_key=3853DA6B

do_build() {
  do_default_build

  if [[ -n "$DO_CHECK" ]]; then
    build_line "Running post-compile tests"
    make check
  fi
}
