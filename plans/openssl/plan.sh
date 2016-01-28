pkg_name=openssl
pkg_derivation=chef
pkg_version=1.0.2f
pkg_license=('BSD')
pkg_source=https://www.openssl.org/source/openssl-1.0.2f.tar.gz
pkg_shasum=932b4ee4def2b434f85435d9e3e19ca8ba99ce9a065a61524b429a9d5e9b2e9c
pkg_gpg_key=3853DA6B
pkg_deps=(chef/glibc chef/zlib chef/cacerts)
pkg_binary_path=(bin)
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
