pkg_name=postgresql
pkg_version=9.5.1
pkg_origin=chef
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('PostgreSQL')
pkg_source=https://ftp.postgresql.org/pub/source/v${pkg_version}/${pkg_name}-${pkg_version}.tar.bz2
pkg_shasum=6b309d8506a39773a752ff074f47656e5424576ea090b04a24fe1725958c5bd2
pkg_gpg_key=3853DA6B

pkg_deps=(
  chef/glibc
  chef/openssl
  chef/perl
  chef/readline
  chef/zlib
)

pkg_build_deps=(
  chef/coreutils
  chef/gcc
  chef/make
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
