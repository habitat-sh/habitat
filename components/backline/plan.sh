pkg_name=hab-backline
pkg_origin=core
pkg_version=$(cat "$PLAN_CONTEXT/../../VERSION")
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('Apache-2.0')
pkg_source=nosuchfile.tar.gz
pkg_build_deps=()

pkg_deps=(
  core/acl
  core/attr
  core/bash
  core/binutils
  core/bzip2
  core/cacerts
  core/coreutils
  core/diffutils
  core/file
  core/findutils
  core/gawk
  core/gcc-libs
  core/glibc
  core/gmp
  core/grep
  core/gzip
  core/hab-plan-build
  core/hab
  core/less
  core/libbsd
  core/libcap
  core/libidn
  core/linux-headers
  core/make
  core/mg
  core/mpfr
  core/ncurses
  core/openssl
  core/pcre
  core/readline
  core/rq
  core/sed
  core/tar
  core/unzip
  core/util-linux
  core/vim
  core/wget
  core/xz
  core/zlib
)

do_download() {
  return 0
}

do_verify() {
  return 0
}

do_unpack() {
  return 0
}

do_prepare() {
  return 0
}

do_build() {
  return 0
}

do_install() {
  return 0
}
