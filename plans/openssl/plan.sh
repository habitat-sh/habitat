pkg_name=openssl
pkg_derivation=chef
pkg_version=1.0.2e
pkg_license=('BSD')
pkg_source=https://www.openssl.org/source/openssl-1.0.2e.tar.gz
pkg_shasum=e23ccafdb75cfcde782da0151731aa2185195ac745eea3846133f2e05c0e0bff
pkg_gpg_key=3853DA6B
pkg_deps=(chef/glibc chef/zlib chef/cacerts)
pkg_binary_path=(bin)
pkg_lib_dirs=(lib)
pkg_include_dirs=(include)

build() {
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
