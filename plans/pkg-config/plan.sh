pkg_name=pkg-config
pkg_derivation=chef
pkg_version=0.29
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_license=('gplv2+')
pkg_source=http://pkgconfig.freedesktop.org/releases/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=c8507705d2a10c67f385d66ca2aae31e81770cc0734b4191eb8c489e864a006b
pkg_build_deps=(chef/binutils chef/gcc)
pkg_deps=(chef/glibc)
pkg_binary_path=(bin)
pkg_gpg_key=3853DA6B

do_build() {
  ./configure \
    --prefix=$pkg_prefix \
    --with-internal-glib \
    --disable-host-tool
  make
}

do_check() {
  make check
}
