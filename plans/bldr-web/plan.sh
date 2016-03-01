pkg_name=bldr-web
pkg_version=0.4.0
pkg_origin=chef
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_license=('Apache2')
pkg_filename=${pkg_name}-${pkg_version}.tar.bz2
pkg_gpg_key=3853DA6B
pkg_deps=(chef/glibc chef/bldr chef/pcre chef/nginx)
pkg_build_deps=(chef/node chef/coreutils chef/phantomjs)
pkg_lib_dirs=(lib)
pkg_include_dirs=(include)
pkg_expose=(80 443)

do_begin() {
    pushd ../../
    mkdir -p $BLDR_SRC_CACHE
    tar -cjvf $BLDR_SRC_CACHE/${pkg_name}-${pkg_version}.tar.bz2 \
		    --transform "s,^\./web,bldr-web-${pkg_version}," ./web
    popd
    pkg_shasum=$(trim $(sha256sum /opt/bldr/cache/src/${pkg_filename} | cut -d " " -f 1))
}

do_build() {
    npm install
    fix_interpreter "${BLDR_SRC_CACHE}/${pkg_name}-${pkg_version}/node_modules/.bin/*" chef/coreutils bin/env
    npm run dist
}

do_install() {
    cp -vR dist ${pkg_path}/dist
}

do_verify() {
    return 0
}

do_download() {
    return 0
}
