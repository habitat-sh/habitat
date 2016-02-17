pkg_name=redis
pkg_derivation=chef
pkg_version=3.0.7
pkg_license=('bsd')
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_source=http://download.redis.io/releases/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=b2a791c4ea3bb7268795c45c6321ea5abcc24457178373e6a6e3be6372737f23
pkg_deps=(chef/glibc)
pkg_build_deps=(chef/coreutils chef/diffutils chef/patch chef/make chef/gcc)
pkg_binary_path=(bin)
pkg_service_run="bin/redis-server /opt/bldr/srvc/redis/config/redis.config"
pkg_docker_build="auto"
pkg_expose=(6379)
pkg_gpg_key=3853DA6B

do_build() {
  make
}
