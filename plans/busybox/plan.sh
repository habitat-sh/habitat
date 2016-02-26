pkg_name=busybox
pkg_distname=$pkg_name
pkg_origin=chef
pkg_version=1.24.1
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_license=('gplv2')
pkg_source=http://www.busybox.net/downloads/${pkg_distname}-${pkg_version}.tar.bz2
pkg_shasum=37d03132cc078937360b392170b7a1d0e5b322eee9f57c0b82292a8b1f0afe3d
pkg_deps=(chef/glibc)
pkg_build_deps=(chef/gcc chef/coreutils chef/sed chef/bison chef/flex chef/grep chef/bash chef/gawk chef/libtool chef/diffutils chef/findutils chef/xz chef/gettext chef/gzip chef/make chef/patch chef/texinfo chef/util-linux chef/wget)
pkg_binary_path=(bin sbin)
pkg_gpg_key=3853DA6B
pkg_interpreters=(bin/ash bin/awk bin/env bin/sh)

do_build() {
  make -j$(nproc)
}

do_prepare() {
  create_config
}

create_config() {
  # To update to a new version, run `make defconfig` to generate a new
  # `.config` file and add the following replacement tokens below.
  cat $PLAN_CONTEXT/config \
    | sed \
      -e "s,@pkg_prefix@,$pkg_prefix,g" \
      -e "s,@pkg_srvc_var@,$pkg_srvc_var,g" \
      -e "s,@cflags@,$CFLAGS,g" \
      -e "s,@ldflags@,$LDFLAGS,g" \
      -e "s,@osname@,bldr,g" \
    > .config
}
