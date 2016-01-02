pkg_name=apr
pkg_derivation=chef
pkg_version=1.5.2
pkg_license=('Apache2')
pkg_source=http://www.us.apache.org/dist/apr/${pkg_name}-${pkg_version}.tar.bz2
pkg_shasum=7d03ed29c22a7152be45b8e50431063736df9e1daa1ddf93f6a547ba7a28f67a
pkg_gpg_key=3853DA6B
pkg_deps=(chef/glibc)
pkg_binary_path=(bin)

do_build() {
  ./configure --prefix=${pkg_prefix}
  make
  make install
}
