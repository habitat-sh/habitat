# shellcheck disable=2154
source "../../../support/ci/builder-base-plan.sh"
pkg_name=hab-eventsrv
pkg_origin=core
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('Apache-2.0')
pkg_bin_dirs=(bin)
pkg_deps=(core/glibc/2.22/20170513201042
          core/gcc-libs/5.2.0/20170513212920
          core/zeromq/4.2.5/20180407102804
          core/libsodium/1.0.13/20170905223149
          core/libarchive/3.3.2/20171018164107)
pkg_build_deps=(core/rust/1.26.2/20180606182054
                core/gcc/5.2.0/20170513202244
                core/pkg-config/0.29/20170513212944
                core/git/2.14.2/20180416203520)
pkg_exports=(
  [consumer_port]=consumer_port
  [producer_port]=producer_port
)
pkg_exposes=(consumer_port producer_port)
bin="eventsrv"
pkg_svc_run="$bin -c ${pkg_svc_config_path}/config.toml"
