pkg_name=bldr-web
pkg_version=0.4.0
pkg_origin=chef
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('Apache2')
pkg_filename=${pkg_name}-${pkg_version}.tar.bz2
pkg_gpg_key=3853DA6B
pkg_deps=(chef/glibc chef/bldr chef/curl chef/pcre chef/nginx)
pkg_build_deps=(chef/node chef/coreutils chef/phantomjs chef/python2
                chef/make chef/gcc chef/gcc-libs)
pkg_lib_dirs=(lib)
pkg_include_dirs=(include)
pkg_expose=(80 443)

do_begin() {
    pushd ../../
    mkdir -p $BLDR_SRC_CACHE
    tar -cjvf $BLDR_SRC_CACHE/${pkg_name}-${pkg_version}.tar.bz2 \
		    --transform "s,^\./web,bldr-web-${pkg_version}," ./web
    popd
    pkg_shasum=$(trim $(sha256sum $BLDR_SRC_CACHE/${pkg_filename} | cut -d " " -f 1))
}

do_build() {
  npm install

  for b in ${BLDR_SRC_CACHE}/${pkg_name}-${pkg_version}/node_modules/.bin/*; do
    fix_interpreter $(readlink -f -n $b) chef/coreutils bin/env
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
