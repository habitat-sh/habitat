pkg_name=gnupg
pkg_derivation=chef
pkg_version=1.4.19
pkg_license=('GPLv3')
pkg_source=https://gnupg.org/ftp/gcrypt/gnupg/${pkg_name}-${pkg_version}.tar.bz2
pkg_shasum=7f09319d044b0f6ee71fe3587bb873be701723ac0952cff5069046a78de8fd86
pkg_gpg_key=3853DA6B
pkg_binary_path=(bin libexec)
pkg_deps=(chef/glibc)

build() {
	./configure --prefix=${pkg_prefix} --with-included-zlib --with-included-regex
	make
}
