pkg_name=perl
pkg_derivation=chef
pkg_version=5.22.0
pkg_license=('GPL' 'PerlArtistic')
pkg_maintainer="Jamie Winsor <reset@chef.io>"
pkg_source=http://www.cpan.org/src/5.0/perl-${pkg_version}.tar.bz2
pkg_filename=${pkg_name}-${pkg_version}.tar.bz2
pkg_deps=(chef/glibc)
pkg_lib_dirs=(lib)
pkg_binary_path=(bin)
pkg_shasum=3689fdb519133d039f49e12834764ff4c1de8f8d925dea44a952ca110d0ac9f7
pkg_gpg_key=3853DA6B

build() {
  ./Configure -de -Dprefix=${pkg_prefix}
  make
}
