source "../../../support/ci/builder-base-plan.sh"
pkg_name=builder-jobsrv
pkg_origin=core
pkg_maintainer="Jamie Winsor <reset@chef.io>"
pkg_license=('Apache-2.0')
pkg_bin_dirs=(bin)
pkg_deps=(core/glibc core/openssl core/gcc-libs core/zeromq core/libsodium core/libarchive)
pkg_build_deps=(core/protobuf core/protobuf-rust core/coreutils core/cacerts
  core/rust core/gcc core/git core/pkg-config)
pkg_exports=(
  [worker_port]=net.worker_command_port
  [worker_heartbeat]=net.worker_heartbeat_port
  [publisher_port]=net.publisher_port
)
pkg_exposes=(worker_port worker_heartbeat publisher_port)
pkg_binds=(
  [router]="port heartbeat"
  [datastore]="port"
)
bin="bldr-job-srv"
pkg_svc_run="$bin start -c ${pkg_svc_path}/config.toml"
