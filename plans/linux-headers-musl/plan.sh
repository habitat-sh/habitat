pkg_name=linux-headers-musl
pkg_derivation=chef
pkg_version=3.12.6-5
pkg_license=('mit')
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_source=https://github.com/sabotage-linux/kernel-headers/archive/v${pkg_version}.tar.gz
pkg_shasum=ecf4db8781dc50a21cbc4cb17b039f96aede53f9da13435a3201373abb49b96b
pkg_dirname=kernel-headers-$pkg_version
pkg_deps=()
pkg_build_deps=(chef/gcc chef/coreutils chef/sed chef/bison chef/flex chef/grep chef/bash chef/gawk chef/libtool chef/diffutils chef/findutils chef/xz chef/gettext chef/gzip chef/make chef/patch chef/texinfo chef/util-linux chef/wget)
pkg_include_dirs=(include)
pkg_gpg_key=3853DA6B

do_build() {
  make \
    ARCH=x86_64 \
    prefix=$pkg_prefix
}

do_install() {
  make \
    ARCH=x86_64 \
    prefix=$pkg_prefix \
    install
}
