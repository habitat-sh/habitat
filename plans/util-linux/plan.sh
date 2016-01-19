pkg_name=util-linux
pkg_derivation=chef
pkg_version=2.27.1
pkg_license=('GPLv2')
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_source=http://ftp.kernel.org/pub/linux/utils/${pkg_name}/v${pkg_version%.?}/${pkg_name}-${pkg_version}.tar.xz
pkg_shasum=0a818fcdede99aec43ffe6ca5b5388bff80d162f2f7bd4541dca94fecb87a290
pkg_deps=(chef/glibc chef/zlib chef/ncurses)
pkg_build_deps=(chef/gcc chef/coreutils chef/sed chef/bison chef/flex chef/grep chef/bash chef/gawk chef/libtool chef/diffutils chef/findutils chef/xz chef/gettext chef/gzip chef/make chef/patch)
pkg_binary_path=(bin)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)
pkg_gpg_key=3853DA6B

do_build() {
  ./configure \
    --prefix=$pkg_prefix \
    --sbindir=$pkg_prefix/bin \
    --localstatedir=$pkg_srvc_var/run \
    --without-python \
    --without-slang \
    --without-systemd \
    --without-systemdsystemunitdir \
    --disable-use-tty-group \
    --disable-chfn-chsh \
    --disable-login \
    --disable-nologin \
    --disable-su \
    --disable-setpriv \
    --disable-runuser \
    --disable-pylibmount
  make
}

do_install() {
  make install usrsbin_execdir=$pkg_path/bin
}
