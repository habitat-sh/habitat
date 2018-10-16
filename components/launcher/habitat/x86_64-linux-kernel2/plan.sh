source "../plan.sh"

pkg_name=hab-launcher
pkg_origin=core
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('Apache-2.0')
pkg_deps=(core/glibc
          core/gcc-libs
          core/libarchive
          core/libsodium
          core/openssl)
pkg_build_deps=(core/coreutils
                core/rust
                core/gcc
                core/git)
