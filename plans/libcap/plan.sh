pkg_name=libcap
pkg_origin=chef
pkg_version=2.24
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('gplv2')
pkg_source=http://ftp.kernel.org/pub/linux/libs/security/linux-privs/libcap2/${pkg_name}-${pkg_version}.tar.xz
pkg_shasum=cee4568f78dc851d726fc93f25f4ed91cc223b1fe8259daa4a77158d174e6c65
pkg_deps=(chef/glibc chef/attr)
pkg_build_deps=(chef/coreutils chef/diffutils chef/patch chef/make chef/gcc chef/linux-headers chef/perl)
pkg_bin_dirs=(bin)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)

do_prepare() {
  do_default_prepare

  # Install binaries under `bin/` vs. `sbin/`
  sed -i "/SBINDIR/s#sbin#bin#" Make.Rules
}

do_build() {
  make KERNEL_HEADERS=$(pkg_path_for linux-headers)/include LDFLAGS="$LDFLAGS"
}

do_install() {
  make prefix=$pkg_prefix lib=lib RAISE_SETFCAP=no install
}


# ----------------------------------------------------------------------------
# **NOTICE:** What follows are implementation details required for building a
# first-pass, "stage1" toolchain and environment. It is only used when running
# in a "stage1" Studio and can be safely ignored by almost everyone. Having
# said that, it performs a vital bootstrapping process and cannot be removed or
# significantly altered. Thank you!
# ----------------------------------------------------------------------------
if [[ "$STUDIO_TYPE" = "stage1" ]]; then
  pkg_build_deps=(chef/gcc chef/linux-headers)
fi
