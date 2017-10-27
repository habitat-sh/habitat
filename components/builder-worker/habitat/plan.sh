source "../../../support/ci/builder-base-plan.sh"
pkg_name=builder-worker
pkg_origin=core
pkg_maintainer="Jamie Winsor <reset@chef.io>"
pkg_license=('Apache-2.0')
pkg_bin_dirs=(bin)
pkg_deps=(core/airlock core/glibc core/openssl core/gcc-libs core/zeromq core/libsodium
  core/libarchive core/zlib core/hab-studio core/hab-pkg-export-docker core/docker core/curl)
pkg_build_deps=(core/make core/cmake core/protobuf core/protobuf-rust core/coreutils core/cacerts
  core/rust core/gcc core/git core/pkg-config)
pkg_binds=(
  [jobsrv]="worker_port worker_heartbeat log_port"
  [depot]="url"
)
pkg_svc_user="root"
pkg_svc_group="root"
bin="bldr-worker"

do_prepare() {
  do_builder_prepare

  # Used by libssh2-sys
  export DEP_Z_ROOT="$(pkg_path_for zlib)"
  export DEP_Z_INCLUDE="$(pkg_path_for zlib)/include"

  # Compile the fully-qualified Studio package identifier into the binary
  PLAN_STUDIO_PKG_IDENT=$(pkg_path_for hab-studio | sed "s,^$HAB_PKG_PATH/,,")
  export PLAN_STUDIO_PKG_IDENT
  build_line "Setting PLAN_STUDIO_PKG_IDENT=$PLAN_STUDIO_PKG_IDENT"

  # Compile the fully-qualified Docker exporter package identifier into the binary
  PLAN_DOCKER_EXPORTER_PKG_IDENT=$(pkg_path_for hab-pkg-export-docker | sed "s,^$HAB_PKG_PATH/,,")
  export PLAN_DOCKER_EXPORTER_PKG_IDENT
  build_line "Setting PLAN_DOCKER_EXPORTER_PKG_IDENT=$PLAN_DOCKER_EXPORTER_PKG_IDENT"

  # Compile the fully-qualified Docker package identifier into the binary
  PLAN_DOCKER_PKG_IDENT=$(pkg_path_for docker | sed "s,^$HAB_PKG_PATH/,,")
  export PLAN_DOCKER_PKG_IDENT
  build_line "Setting PLAN_DOCKER_PKG_IDENT=$PLAN_DOCKER_PKG_IDENT"
}
