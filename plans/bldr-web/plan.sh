pkg_name=bldr-web
pkg_version=0.4.0
pkg_origin=core
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('Apache2')
pkg_filename=${pkg_name}-${pkg_version}.tar.bz2
pkg_deps=(core/glibc core/bldr core/curl core/pcre core/nginx)
pkg_build_deps=(core/node core/coreutils core/phantomjs core/python2
                core/make core/gcc core/gcc-libs)
pkg_lib_dirs=(lib)
pkg_include_dirs=(include)
pkg_expose=(80 443)

do_begin() {
    pushd ../../
    mkdir -p $HAB_CACHE_SRC_PATH
    tar -cjvf $HAB_CACHE_SRC_PATH/${pkg_name}-${pkg_version}.tar.bz2 \
		    --transform "s,^\./web,bldr-web-${pkg_version}," ./web
    popd
    pkg_shasum=$(trim $(sha256sum $HAB_CACHE_SRC_PATH/${pkg_filename} | cut -d " " -f 1))
}

do_build() {
  npm install

  for b in ${HAB_CACHE_SRC_PATH}/${pkg_name}-${pkg_version}/node_modules/.bin/*; do
    fix_interpreter $(readlink -f -n $b) core/coreutils bin/env
  done

  npm run postinstall
  npm run dist
}

do_install() {
    cp -vR dist ${pkg_prefix}/dist
}

do_verify() {
    return 0
}

do_download() {
    return 0
}
