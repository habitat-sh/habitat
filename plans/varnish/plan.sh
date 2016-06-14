pkg_origin=core
pkg_name=varnish
pkg_version='4.1.2'
pkg_maintainer='The Habitat Maintainers <humans@habitat.sh>'
pkg_license=('MIT')
pkg_source=http://repo.varnish-cache.org/source/varnish-4.1.2.tar.gz
pkg_shasum=9728da944d28eb5be90e7ab6799c2c4c831ef4df5e5154537eb7f2e5d5e348c4

pkg_include_dirs=(include)
pkg_lib_dirs=(lib)
pkg_bin_dirs=(bin)

pkg_deps=(
  core/gcc
  core/glibc
  core/pcre
  core/ncurses
)

pkg_build_deps=(
  core/make
  core/python2
  core/docutils
  core/libedit
  core/readline
)

pkg_svc_run="bin/varnishd -d -f $pkg_svc_config_path/varnish.config -n $pkg_svc_var_path/habitat_varnish"

do_build() {
  PCRE_LIBS="-L$(pkg_path_for core/pcre)/lib -lpcre" \
  PCRE_CFLAGS="-I$(pkg_path_for core/pcre)/include" \
  ./configure --prefix=$pkg_prefix --sbindir=${pkg_prefix}/bin
  make
}

do_install() {
  do_default_install
}

do_begin() {
  do_default_begin
  mkdir -p $pkg_svc_var_path/habitat_varnish
  chmod 777 $pkg_svc_var_path/habitat_varnish
}
