pkg_name=iptables
pkg_derivation=chef
pkg_version=1.6.0
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_license=('gplv2')
pkg_source="http://netfilter.org/projects/iptables/files/${pkg_name}-${pkg_version}.tar.bz2"
pkg_shasum=4bb72a0a0b18b5a9e79e87631ddc4084528e5df236bc7624472dcaa8480f1c60
pkg_deps=(chef/glibc)
pkg_build_deps=(chef/make chef/gcc chef/bison chef/flex)
pkg_binary_path=(bin sbin)
pkg_lib_dirs=(lib)
pkg_include_dirs=(include)
pkg_gpg_key=3853DA6B

do_build() {
  ./configure \
    --prefix=$pkg_prefix \
    --enable-devel \
    --disable-static \
    --enable-shared \
    --enable-libipq \
    --disable-nftables \
    --with-xtlibdir=$pkg_prefix/lib/xtlibs
  make
}
