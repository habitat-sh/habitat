source "../../../support/ci/builder-base-plan.sh"
pkg_name=builder-sessionsrv
pkg_origin=core
pkg_maintainer="Jamie Winsor <reset@chef.io>"
pkg_license=('Apache-2.0')
pkg_bin_dirs=(bin)
pkg_deps=(core/glibc core/openssl core/gcc-libs core/zeromq core/libsodium core/libarchive
  core/postgresql)
pkg_build_deps=(core/protobuf core/protobuf-rust core/coreutils core/cacerts
  core/rust core/gcc core/git core/pkg-config)
pkg_binds=(
  [router]="port"
  [datastore]="port"
)
bin="bldr-sessionsrv"
