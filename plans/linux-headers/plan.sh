pkg_name=linux-headers
pkg_origin=chef
pkg_version=4.3
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_license=('gplv2')
pkg_source=http://ftp.kernel.org/pub/linux/kernel/v4.x/linux-${pkg_version}.tar.xz
pkg_shasum=4a622cc84b8a3c38d39bc17195b0c064d2b46945dfde0dae18f77b120bc9f3ae
pkg_dirname=linux-$pkg_version
pkg_deps=()
pkg_build_deps=(chef/coreutils chef/diffutils chef/patch chef/make chef/gcc)
pkg_include_dirs=(include)
pkg_gpg_key=3853DA6B

do_build() {
  make headers_install ARCH=x86 INSTALL_HDR_PATH=$pkg_prefix
}

do_install() {
  find $pkg_prefix/include \( -name ..install.cmd -o -name .install \) -print0 | xargs -0 rm -v
}


# ----------------------------------------------------------------------------
# **NOTICE:** What follows are implementation details required for building a
# first-pass, "stage1" toolchain and environment. It is only used when running
# in a "stage1" Studio and can be safely ignored by almost everyone. Having
# said that, it performs a vital bootstrapping process and cannot be removed or
# significantly altered. Thank you!
# ----------------------------------------------------------------------------
if [[ "$STUDIO_TYPE" = "stage1" ]]; then
  pkg_build_deps=()
fi
