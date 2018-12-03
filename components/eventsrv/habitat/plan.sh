# shellcheck disable=2154
source "../../../support/ci/builder-base-plan.sh"
pkg_name=hab-eventsrv
pkg_origin=core
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('Apache-2.0')
pkg_bin_dirs=(bin)
pkg_deps=(core/glibc
          core/gcc-libs
          core/zeromq
          core/libsodium
          core/libarchive)
pkg_build_deps=(core/rust
                core/gcc
                core/pkg-config
                core/git)
pkg_exports=(
  [consumer_port]=consumer_port
  [producer_port]=producer_port
)
pkg_exposes=(consumer_port producer_port)
bin="eventsrv"
pkg_svc_run="$bin -c ${pkg_svc_config_path}/config.toml"
