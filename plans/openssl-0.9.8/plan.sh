pkg_origin=core
pkg_name=openssl-0.9.8
pkg_version=0.9.8zg
pkg_license=('BSD')
pkg_source=http://openssl.org/source/openssl-${pkg_version}.tar.gz
pkg_dirname=openssl-0.9.8zg
pkg_shasum=06500060639930e471050474f537fcd28ec934af92ee282d78b52460fbe8f580
pkg_deps=(core/glibc core/zlib core/cacerts)
pkg_bin_dirs=(bin)
pkg_lib_dirs=(lib)
pkg_include_dirs=(include)

do_build() {
	./config \
		--prefix=${pkg_prefix} \
		no-idea \
		no-mdc2 \
		no-rc5 \
		zlib \
		shared \
		disable-gost \
		$CFLAGS \
		$LDFLAGS \
		-Wl,-rpath,$LD_RUN_PATH
	make depend
	make
}

