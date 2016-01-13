pkg_name=postgresql
pkg_derivation=chef
pkg_version=9.5.0
pkg_license=('BSD')
pkg_maintainer="Irving Popovetsky <irving@chef.io>"
pkg_source=https://ftp.postgresql.org/pub/source/v${pkg_version}/${pkg_name}-${pkg_version}.tar.bz2
pkg_shasum=f1c0d3a1a8aa8c92738cab0153fbfffcc4d4158b3fee84f7aa6bfea8283978bc
pkg_gpg_key=3853DA6B
pkg_deps=(chef/glibc chef/perl chef/openssl)
pkg_service_run="bin/pgstarter"
pkg_docker_build="auto"
pkg_expose=(5432)

do_build() {
	# ld manpage: " If -rpath is not used when linking an ELF executable, the contents of the environment variable LD_RUN_PATH will be used if it is defined"
	./configure --without-readline --disable-rpath --with-openssl --prefix=$pkg_prefix --sysconfdir=$pkg_srvc_config --localstatedir=$pkg_srvc_data
	make
}

do_install() {
	make install

	cat <<EOT >> $pkg_path/bin/pgstarter
#!/bin/sh
PGCONFIG=/opt/bldr/srvc/postgresql/config/postgresql.conf
PKG_PREFIX=$pkg_prefix
export PGDATA=/opt/bldr/srvc/postgresql/data

if [[ ! -f "\$PGDATA/PG_VERSION" ]]; then
	$pkg_prefix/bin/initdb && $pkg_prefix/bin/postmaster -c config_file=\$PGCONFIG
else
	$pkg_prefix/bin/postmaster -c config_file=\$PGCONFIG
fi

EOT

	chmod +x $pkg_path/bin/pgstarter
}
