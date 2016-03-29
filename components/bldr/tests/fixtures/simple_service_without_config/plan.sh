pkg_name=simple_service_without_config
pkg_origin=test
pkg_version=0.0.1
pkg_license=('Apache2')
pkg_maintainer="Adam Jacob <adam@chef.io>"
pkg_source=http://example.com/releases/${pkg_name}-${pkg_version}.tar.bz2
pkg_filename=${pkg_name}-${pkg_version}.tar.bz2
pkg_shasum=0e21be5d7c5e6ab6adcbed257619897db59be9e1ded7ef6fd1582d0cdb5e5bb7
pkg_gpg_key=3853DA6B
pkg_bin_dirs=(bin)
pkg_deps=(chef/gpgme chef/libassuan chef/libgpg-error)
pkg_service_run="bin/simple_service_without_config"
pkg_docker_build="auto"
pkg_docker_build_no_cache="true"

do_begin() {
	tar -cjvf $BLDR_SRC_CACHE/${pkg_name}-${pkg_version}.tar.bz2 --exclude 'plans' --exclude '.git' --exclude '.gitignore' --exclude 'target' --transform "s,^\.,simple_service_without_config-0.0.1," .
	pkg_shasum=$(trim $(sha256sum /opt/bldr/cache/src/simple_service_without_config-0.0.1.tar.bz2 | cut -d " " -f 1))
}

do_download() {
	return 0
}

do_build() {
	return 0
}

do_install() {
  cp -r /src/components/bldr/target/debug/bldr $pkg_prefix/bin
	cp -r $BLDR_SRC_CACHE/$pkg_dirname/bin $pkg_prefix
	chmod 755 $pkg_path/bin
    chmod 755 $pkg_path/bin/*
	return 0
}
