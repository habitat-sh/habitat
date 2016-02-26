pkg_name=check
pkg_origin=chef
pkg_version=0.10.0
pkg_license=('lgplv2+')
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_source=http://downloads.sourceforge.net/sourceforge/$pkg_name/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=f5f50766aa6f8fe5a2df752666ca01a950add45079aa06416b83765b1cf71052
pkg_deps=(chef/glibc chef/gawk)
pkg_build_deps=(chef/coreutils chef/diffutils chef/patch chef/make chef/gcc chef/sed)
pkg_binary_path=(bin)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)
pkg_gpg_key=3853DA6B

do_install() {
  do_default_install

  # Clean up extra files
  rm -rfv $pkg_path/share/info/dir $pkg_path/share/doc/check/*ChangeLog*
}


# ----------------------------------------------------------------------------
# **NOTICE:** What follows are implementation details required for building a
# first-pass, "stage1" toolchain and environment. It is only used when running
# in a "stage1" Studio and can be safely ignored by almost everyone. Having
# said that, it performs a vital bootstrapping process and cannot be removed or
# significantly altered. Thank you!
# ----------------------------------------------------------------------------
if [[ "$STUDIO_TYPE" = "stage1" ]]; then
  pkg_build_deps=(chef/gcc chef/coreutils chef/sed chef/diffutils chef/make chef/patch)
fi
