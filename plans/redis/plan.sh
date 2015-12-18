pkg_name=redis
pkg_derivation=chef
pkg_version=3.0.1
pkg_license=('BSD')
pkg_maintainer="Adam Jacob <adam@chef.io>"
pkg_source=http://download.redis.io/releases/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=0e21be5d7c5e6ab6adcbed257619897db59be9e1ded7ef6fd1582d0cdb5e5bb7
pkg_gpg_key=3853DA6B
pkg_binary_path=(bin)
pkg_deps=(chef/glibc)
pkg_service_run="bin/redis-server /opt/bldr/srvc/redis/config/redis.config"
pkg_docker_build="auto"
pkg_expose=(6379)

do_build() {
	make
}
