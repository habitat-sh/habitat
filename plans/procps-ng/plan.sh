pkg_name=procps-ng
pkg_derivation=chef
pkg_version=3.3.11
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_license=('gpl' 'lgpl')
pkg_source=http://downloads.sourceforge.net/project/${pkg_name}/Production/${pkg_name}-${pkg_version}.tar.xz
pkg_shasum=e9493169a2d2adc0bc045538707310c8e877b385e4e296143b62607d2bb044ed
pkg_deps=(chef/glibc chef/ncurses)
pkg_build_deps=(chef/binutils chef/gcc)
pkg_binary_path=(bin)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)
pkg_gpg_key=3853DA6B

do_build() {
  # The Util-linux package will provide the `kill` command
  ./configure \
    --prefix=$pkg_prefix \
    --sbindir=$pkg_prefix/bin \
    --disable-kill
  make

  if [[ -n "$DO_CHECK" ]]; then
    make check
  fi
}
