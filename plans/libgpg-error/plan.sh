pkg_name=libgpg-error
pkg_origin=core
pkg_version=1.20
pkg_license=('lgplv2+')
pkg_maintainer="The Habitat Contributors <humans@habitat.sh>"
pkg_source=ftp://ftp.gnupg.org/gcrypt/${pkg_name}/${pkg_name}-${pkg_version}.tar.bz2
pkg_shasum=3266895ce3419a7fb093e63e95e2ee3056c481a9bc0d6df694cfd26f74e72522
pkg_deps=(core/glibc)
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
