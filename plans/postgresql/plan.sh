pkg_name=postgresql
pkg_version=9.5.3
pkg_origin=core
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('PostgreSQL')
pkg_source=https://ftp.postgresql.org/pub/source/v${pkg_version}/${pkg_name}-${pkg_version}.tar.bz2
pkg_shasum=7385c01dc58acba8d7ac4e6ad42782bd7c0b59272862a3a3d5fe378d4503a0b4

pkg_deps=(
  core/glibc
  core/openssl
  core/perl
  core/readline
  core/zlib
)

pkg_build_deps=(
  core/coreutils
  core/gcc
  core/make
)

pkg_bin_dirs=(bin)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)
pkg_expose=(5432)

do_build() {
	# ld manpage: "If -rpath is not used when linking an ELF
	# executable, the contents of the environment variable LD_RUN_PATH
	# will be used if it is defined"
	./configure --disable-rpath \
              --with-openssl \
              --prefix=${pkg_prefix} \
              --sysconfdir=${pkg_svc_config_path} \
              --localstatedir=${pkg_svc_var_path}
	make
}

do_install() {
	make install
}
