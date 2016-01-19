pkg_name=gawk
pkg_derivation=chef
pkg_version=4.1.3
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_license=('gplv3+')
pkg_source=http://ftp.gnu.org/gnu/$pkg_name/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=524effa5b9ecd4ed940f2581c5d3c1df4e4bd7e6f768aa033c1916f47dfc6e29
pkg_deps=(chef/glibc chef/mpfr)
pkg_build_deps=(chef/gcc chef/coreutils chef/sed chef/bison chef/flex chef/grep chef/bash)
pkg_binary_path=(bin)
pkg_gpg_key=3853DA6B

do_build() {
  do_default_build

  if [[ -n "$DO_CHECK" ]]; then
    build_line "Running post-compile tests"
    make check
  fi
}
