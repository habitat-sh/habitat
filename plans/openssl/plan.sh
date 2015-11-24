pkg_name=openssl
pkg_derivation=chef
pkg_version=1.0.2d
pkg_license=('BSD')
pkg_source=https://www.openssl.org/source/openssl-1.0.2d.tar.gz
pkg_shasum=671c36487785628a703374c652ad2cebea45fa920ae5681515df25d9f2c9a8c8
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
