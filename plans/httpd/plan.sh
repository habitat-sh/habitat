pkg_name=httpd
pkg_origin=chef
pkg_version=2.4.18
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('apache')
pkg_source=http://www.apache.org/dist/${pkg_name}/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=1c39b55108223ba197cae2d0bb81c180e4db19e23d177fba5910785de1ac5527
pkg_deps=(chef/glibc chef/expat chef/libiconv chef/apr chef/apr-util chef/pcre chef/zlib chef/openssl)
pkg_build_deps=(chef/patch chef/make chef/gcc)
pkg_bin_dirs=(bin)
pkg_lib_dirs=(lib)
pkg_gpg_key=3853DA6B
pkg_expose=(80 443)
pkg_service_run="bin/httpd -DFOREGROUND -f $pkg_svc_config/httpd.conf"
pkg_service_user="root"

do_build() {
  ./configure --prefix=$pkg_prefix \
              --with-expat=$(pkg_path_for chef/expat) \
              --with-iconv=$(pkg_path_for chef/libiconv) \
              --with-pcre=$(pkg_path_for chef/pcre) \
              --with-apr=$(pkg_path_for chef/apr) \
              --with-apr-util=$(pkg_path_for chef/apr-util) \
              --with-z=$(pkg_path_for chef/zlib) \
              --enable-ssl --with-ssl=$(pkg_path_for chef/openssl) \
              --enable-modules=most --enable-mods-shared=most
  make
}
