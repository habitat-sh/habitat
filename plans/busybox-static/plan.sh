source ../busybox/plan.sh

pkg_name=busybox-static
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_dirname=${pkg_distname}-${pkg_version}

pkg_deps=()
pkg_build_deps=(chef/linux-headers-musl chef/musl "${pkg_build_deps[@]}")
pkg_gpg_key=3853DA6B

do_prepare() {
  CFLAGS="-I$(pkg_path_for linux-headers-musl)/include -I$(pkg_path_for musl)/include"
  build_line "Overriding CFLAGS=$CFLAGS"

  PLAN_CONTEXT=$PLAN_CONTEXT/../busybox create_config
  sed \
    -e '/CONFIG_STATIC/ s,^.*$,CONFIG_STATIC=y,' \
    -e '/CONFIG_FEATURE_UTMP/ s,^.*$,CONFIG_FEATURE_UTMP=n,' \
    -e '/CONFIG_FEATURE_WTMP/ s,^.*$,CONFIG_FEATURE_WTMP=n,' \
    -e '/CONFIG_INETD/ s,^.*$,CONFIG_INETD=n,' \
    -i .config

  sed '1,1i#include <sys/resource.h>' -i include/libbb.h
}

do_build() {
  make -j$(nproc) CC=musl-gcc
}

do_install() {
  install -Dm755 busybox $pkg_path/bin/busybox

  # Generate the symlinks back to the `busybox` executable
  for l in $($pkg_path/bin/busybox --list); do
    ln -sv busybox $pkg_path/bin/$l
  done
}
