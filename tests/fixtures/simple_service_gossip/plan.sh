pkg_name=simple_service_gossip
pkg_derivation=test
pkg_version=0.0.1
pkg_license=('Apache2')
pkg_maintainer="Adam Jacob <adam@chef.io>"
pkg_source=http://example.com/releases/${pkg_name}-${pkg_version}.tar.bz2
pkg_filename=${pkg_name}-${pkg_version}.tar.bz2
pkg_shasum=788f3a67b0f4ff594a54d465c8e8db55635d591ed6aa836014834602a6f59918
pkg_gpg_key=3853DA6B
pkg_binary_path=(bin)
pkg_deps=(chef/gpgme chef/libassuan chef/libgpg-error chef/libarchive chef/busybox)
pkg_service_run="bin/simple_service"
pkg_docker_build="auto"
pkg_docker_build_no_cache="true"
pkg_docker_from="chef/bldr:latest"
pkg_service_user=root

do_begin() {
	tar -cjvf $BLDR_SRC_CACHE/${pkg_name}-${pkg_version}.tar.bz2 --exclude 'plans' --exclude '.git' --exclude '.gitignore' --exclude 'target' --transform "s,^\.,simple_service_gossip-0.0.1," .
	pkg_shasum=$(trim $(sha256sum /opt/bldr/cache/src/simple_service_gossip-0.0.1.tar.bz2 | cut -d " " -f 1))
}

do_download() {
	return 0
}

do_build() {
	return 0
}

do_install() {
	cp -r $BLDR_SRC_CACHE/$pkg_dirname/bin $pkg_prefix
	chmod 755 $pkg_path/bin
  chmod 755 $pkg_path/bin/*
	return 0
}
