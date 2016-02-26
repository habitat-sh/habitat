pkg_name=libaio
pkg_origin=chef
pkg_version=0.3.109
pkg_license=('LGPL')
pkg_maintainer="Jamie Winsor <reset@chef.io>"
pkg_source=http://ftp.de.debian.org/debian/pool/main/liba/${pkg_name}/${pkg_name}_${pkg_version}.orig.tar.gz
pkg_filename=${pkg_name}-${pkg_version}.tar.gz
pkg_lib_dirs=(lib)
pkg_include_dirs=(include)
pkg_shasum=bf4a457253cbaab215aea75cb6e18dc8d95bbd507e9920661ff9bdd288c8778d
pkg_gpg_key=3853DA6B

do_build() {
  make
}

do_install() {
  make install prefix=${pkg_prefix}
}
