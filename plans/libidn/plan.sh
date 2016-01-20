pkg_name=libidn
pkg_derivation=chef
pkg_version=1.32
pkg_license=('lgplv2+')
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_source=http://ftp.gnu.org/gnu/$pkg_name/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=ba5d5afee2beff703a34ee094668da5c6ea5afa38784cebba8924105e185c4f5
pkg_deps=(chef/glibc)
pkg_build_deps=(chef/gcc chef/sed chef/bison chef/flex chef/grep chef/bash chef/libtool chef/diffutils chef/findutils chef/xz chef/gettext chef/gzip chef/make chef/patch chef/texinfo chef/util-linux)
pkg_binary_path=(bin)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)
pkg_gpg_key=3853DA6B

do_check() {
  make check
}
