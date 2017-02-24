source "../../../support/ci/builder-base-plan.sh"
pkg_name=builder-worker
pkg_origin=core
pkg_maintainer="Jamie Winsor <reset@chef.io>"
pkg_license=('Apache-2.0')
pkg_bin_dirs=(bin)
pkg_deps=(core/glibc core/openssl core/gcc-libs core/zeromq core/libsodium core/libarchive
  core/zlib core/hab-studio core/curl)
pkg_build_deps=(core/make core/cmake core/protobuf core/protobuf-rust core/coreutils core/cacerts
  core/rust core/gcc core/git core/pkg-config)
bin="bldr-worker"
pkg_svc_run="$bin start -c ${pkg_svc_path}/config.toml"
pkg_svc_user="root"
pkg_svc_group="root"
pkg_binds=(
  [jobsrv]="worker_port worker_heartbeat publisher_port"
)

do_prepare() {
  do_builder_prepare

  # Used by libssh2-sys
  export DEP_Z_ROOT="$(pkg_path_for zlib)"
  export DEP_Z_INCLUDE="$(pkg_path_for zlib)/include"
}
