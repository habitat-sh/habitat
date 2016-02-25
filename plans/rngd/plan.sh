pkg_name=rngd
pkg_derivation=chef
pkg_version=5
pkg_maintainer="Dave Parfitt <dparfitt@chef.io>"
pkg_license=('gplv2')
pkg_source=http://http.debian.net/debian/pool/main/r/rng-tools/rng-tools_2-unofficial-mt.14.orig.tar.bz2
pkg_shasum=a3791d566106873c361e19819f79c4fff44514cdf65c10f8a16e9ee3840f04ee
pkg_deps=(chef/glibc)
pkg_build_deps=(chef/coreutils chef/autoconf chef/automake chef/make chef/gcc)
# package has a _ but the extracted dir has a -
pkg_dirname=rng-tools-2-unofficial-mt.14
pkg_binary_path=(bin)
pkg_service_run="sbin/rngd -f -r /dev/urandom"
pkg_service_user=root
pkg_gpg_key=3853DA6B

do_build() {
  ./autogen.sh
  ./configure --prefix=$pkg_prefix
  make
}

