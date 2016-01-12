pkg_name=libcap
pkg_derivation=chef
pkg_version=2.24
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_license=('gplv2')
pkg_source=http://ftp.kernel.org/pub/linux/libs/security/linux-privs/libcap2/${pkg_name}-${pkg_version}.tar.xz
pkg_shasum=cee4568f78dc851d726fc93f25f4ed91cc223b1fe8259daa4a77158d174e6c65
pkg_build_deps=(chef/binutils chef/gcc chef/linux-headers)
pkg_deps=(chef/glibc chef/attr)
pkg_binary_path=(bin)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)
pkg_gpg_key=3853DA6B

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
