pkg_name=libbsd
pkg_derivation=chef
pkg_version=0.8.1
pkg_license=('custom')
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_source=http://libbsd.freedesktop.org/releases/${pkg_name}-${pkg_version}.tar.xz
pkg_shasum=adbc8781ad720bce939b689f38a9f0247732a36792147a7c28027c393c2af9b0
pkg_deps=(chef/glibc)
pkg_build_deps=(chef/gcc chef/coreutils chef/sed chef/bison chef/flex chef/grep chef/bash chef/gawk chef/libtool chef/diffutils chef/findutils chef/xz chef/gettext chef/gzip chef/make chef/patch chef/texinfo chef/util-linux)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)
pkg_gpg_key=3853DA6B

do_install() {
  do_default_install

  # Install license file from README
  install -Dm644 COPYING "$pkg_path/share/licenses/LICENSE"
}
