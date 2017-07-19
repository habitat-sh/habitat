source "../../../support/ci/builder-base-plan.sh"
pkg_name=builder-scheduler
pkg_origin=core
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('Apache-2.0')
pkg_bin_dirs=(bin)
pkg_deps=(core/glibc core/openssl core/gcc-libs core/zeromq core/libsodium core/libarchive
  core/postgresql)
pkg_build_deps=(core/protobuf core/protobuf-rust core/coreutils core/cacerts
  core/rust core/gcc core/git core/pkg-config)
pkg_binds=(
  [router]="port heartbeat"
  [datastore]="port"
  [depot]="url"
)
bin="bldr-scheduler"
pkg_svc_run="$bin start -c ${pkg_svc_config_path}/config.toml"
