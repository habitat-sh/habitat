pkg_name=bldr-web
pkg_version=0.4.0
pkg_origin=chef
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_license=('Apache2')
pkg_source=http://example.com/${pkg_name}-${pkg_version}.tar.bz2
pkg_filename=${pkg_name}-${pkg_version}.tar.bz2
pkg_shasum=01ba4719c80b6fe911b091a7c05124b64eeece964e09c058ef8f9805daca546b
pkg_gpg_key=3853DA6B
pkg_deps=(chef/glibc chef/bldr chef/pcre chef/nginx)
pkg_build_deps=(chef/node)
pkg_lib_dirs=(lib)
pkg_include_dirs=(include)
pkg_docker_build="auto"
pkg_expose=(80 443)

do_begin() {
    pushd ../../
    tar -cjvf $BLDR_SRC_CACHE/${pkg_name}-${pkg_version}.tar.bz2 \
		    --transform "s,^\./web,bldr-web-$pkg_version," ./web
    popd
    pkg_shasum=$(trim $(sha256sum /opt/bldr/cache/src/${pkg_filename} | cut -d " " -f 1))
}

do_build() {
    npm install
    npm run dist
}

do_install() {
    cp -R dist ${pkg_path}/dist
}

do_verify() {
    return 0
}

do_download() {
    return 0
}
