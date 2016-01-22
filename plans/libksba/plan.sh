pkg_name=libksba
pkg_derivation=chef
pkg_version=1.3.3
pkg_license=('lgplv3+')
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_source=ftp://ftp.gnupg.org/gcrypt/${pkg_name}/${pkg_name}-${pkg_version}.tar.bz2
pkg_shasum=0c7f5ffe34d0414f6951d9880a46fcc2985c487f7c36369b9f11ad41131c7786
pkg_deps=(chef/glibc chef/libgpg-error)
pkg_build_deps=(chef/gcc chef/coreutils chef/sed chef/bison chef/flex chef/grep chef/bash chef/gawk chef/libtool chef/diffutils chef/findutils chef/xz chef/gettext chef/gzip chef/make chef/patch chef/texinfo chef/util-linux)
pkg_binary_path=(bin)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)
pkg_gpg_key=3853DA6B

do_build() {
  ./configure \
    --prefix=${pkg_prefix} \
    --enable-static \
    --enable-shared
  make
}

do_check() {
  make check
}
