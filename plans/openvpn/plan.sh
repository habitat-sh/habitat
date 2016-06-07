pkg_name=openvpn
pkg_origin=core
pkg_version=2.3.11
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('gplv2')
pkg_source=https://swupdate.openvpn.org/community/releases/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=9117a4434fd35e61cf94f9ee7ef84b7aecbc6fa556f779ff599560f219756163
pkg_deps=(core/glibc core/openssl core/lzo)
pkg_build_deps=(core/gcc core/coreutils core/make core/busybox-static)
pkg_bin_dirs=(bin)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)

do_build() {
  ./configure \
    --disable-plugin-auth-pam \
    --prefix=${pkg_prefix} \
    --exec-prefix=${pkg_prefix} \
    --sbindir=${pkg_prefix}/bin \
    --enable-iproute2
  make
}
