pkg_name=busybox
pkg_distname=$pkg_name
pkg_origin=chef
pkg_version=1.24.2
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('gplv2')
pkg_source=http://www.busybox.net/downloads/${pkg_distname}-${pkg_version}.tar.bz2
pkg_shasum=e71ef53ec656f31c42633918d301405d40dea1d97eca12f272217ae4a971c855

pkg_deps=(chef/glibc)
pkg_build_deps=(
  chef/bash
  chef/bison
  chef/coreutils
  chef/diffutils
  chef/findutils
  chef/flex
  chef/gawk
  chef/gcc
  chef/gettext
  chef/grep
  chef/gzip
  chef/libtool
  chef/make
  chef/patch
  chef/sed
  chef/texinfo
  chef/util-linux
  chef/wget
  chef/xz
)

pkg_bin_dirs=(bin)
pkg_gpg_key=3853DA6B
pkg_interpreters=(bin/ash bin/awk bin/env bin/sh bin/bash)

do_prepare() {
  create_config
}

do_build() {
  make -j$(nproc)
}

do_install() {
  install -Dm755 busybox $pkg_prefix/bin/busybox

  # Generate the symlinks back to the `busybox` executable
  for l in $($pkg_prefix/bin/busybox --list); do
    ln -sv busybox $pkg_prefix/bin/$l
  done
}

create_config() {
  # To update to a new version, run `make defconfig` to generate a new
  # `.config` file and add the following replacement tokens below.
  build_line "Customizing busybox configuration..."
  cat $PLAN_CONTEXT/config \
    | sed \
      -e "s,@pkg_prefix@,$pkg_prefix,g" \
      -e "s,@pkg_svc_var@,$pkg_svc_var_path,g" \
      -e "s,@cflags@,$CFLAGS,g" \
      -e "s,@ldflags@,$LDFLAGS,g" \
      -e "s,@osname@,bldr,g" \
      -e "s,@bash_is_ash@,y,g" \
    > .config
}
