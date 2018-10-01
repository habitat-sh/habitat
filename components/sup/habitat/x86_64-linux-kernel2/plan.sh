# shellcheck disable=2034,2154
source "../plan.sh"

pkg_name=hab-sup
_pkg_distname=$pkg_name
pkg_origin=core
pkg_version=$(cat "$SRC_PATH/../../VERSION")
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('Apache-2.0')
pkg_deps=(core/busybox-static
          core/glibc
          core/gcc-libs
          core/libarchive
          core/libsodium
          core/openssl
          core/zeromq)
pkg_build_deps=(core/coreutils
                core/cacerts
                core/rust
                core/gcc
                core/raml2html)
