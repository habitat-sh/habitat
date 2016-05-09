pkg_name=erlang
pkg_origin=core
pkg_version=18.3
pkg_dirname=otp_src_${pkg_version}
pkg_license=('erlang')
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_source=http://www.erlang.org/download/otp_src_${pkg_version}.tar.gz
pkg_filename=otp_src_${pkg_version}.tar.gz
pkg_shasum=fdab8129a1cb935db09f1832e3a7d511a4aeb2b9bb3602ca6a7ccb9730d5c9c3
pkg_deps=(core/glibc core/zlib core/ncurses)
pkg_build_deps=(core/coreutils core/gcc core/make core/perl)
pkg_bin_dirs=(bin)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)

do_prepare() {
  # The `/bin/pwd` path is hardcoded, so we'll add a symlink if needed.
  if [[ ! -r /bin/pwd ]]; then
    ln -sv $(pkg_path_for coreutils)/bin/pwd /bin/pwd
    _clean_pwd=true
  fi
}

do_build() {
  ./configure --prefix=${pkg_prefix} \
              --enable-threads \
              --enable-smp-support \
              --enable-kernel-poll \
              --enable-threads \
              --enable-smp-support \
              --enable-kernel-poll \
              --enable-dynamic-ssl-lib \
              --enable-shared-zlib \
              --enable-hipe \
              --without-javac \
              --disable-debug
  make
}

do_end() {
  # Clean up the `pwd` link, if we set it up.
  if [[ -n "$_clean_pwd" ]]; then
    rm -fv /bin/pwd
  fi
}
