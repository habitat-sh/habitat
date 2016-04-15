pkg_name=redis
pkg_origin=core
pkg_version=3.0.7
pkg_license=('BSD')
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_source=http://download.redis.io/releases/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=b2a791c4ea3bb7268795c45c6321ea5abcc24457178373e6a6e3be6372737f23
pkg_bin_dirs=(bin)
pkg_build_deps=(core/make core/gcc)
pkg_deps=(core/glibc)
pkg_service_run="bin/redis-server $pkg_svc_config_path/redis.config"
pkg_expose=(6379)

do_build() {
	make
}
