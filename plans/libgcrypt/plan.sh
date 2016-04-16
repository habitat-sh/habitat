pkg_name=libgcrypt
pkg_origin=core
pkg_version=1.6.4
pkg_license=('lgplv2+')
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_source=ftp://ftp.gnupg.org/gcrypt/${pkg_name}/${pkg_name}-${pkg_version}.tar.bz2
pkg_shasum=c9bc2c7fe2e5f4ea13b0c74f9d24bcbb1ad889bb39297d8082aebf23f4336026
pkg_deps=(core/glibc core/libgpg-error)
pkg_build_deps=(core/gcc core/coreutils core/sed core/bison core/flex core/grep core/bash core/gawk core/libtool core/diffutils core/findutils core/xz core/gettext core/gzip core/make core/patch core/texinfo core/util-linux)
pkg_bin_dirs=(bin)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)

do_build() {
  ./configure \
    --prefix=${pkg_prefix} \
    --enable-static \
    --enable-shared
  make
}

do_check() {
  make check
}
