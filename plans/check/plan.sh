pkg_name=check
pkg_derivation=chef
pkg_version=0.10.0
pkg_license=('lgplv2+')
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_source=http://downloads.sourceforge.net/sourceforge/$pkg_name/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=f5f50766aa6f8fe5a2df752666ca01a950add45079aa06416b83765b1cf71052
pkg_deps=(chef/glibc chef/gawk)
pkg_build_deps=(chef/gcc chef/sed chef/bison chef/flex chef/grep chef/bash chef/libtool chef/diffutils chef/findutils chef/xz chef/gettext chef/gzip chef/make chef/patch chef/texinfo chef/util-linux)
pkg_binary_path=(bin)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)
pkg_gpg_key=3853DA6B

do_install() {
  do_default_install

  # Clean up extra files
  rm -rfv $pkg_path/share/info/dir $pkg_path/share/doc/check/*ChangeLog*
}
