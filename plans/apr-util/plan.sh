pkg_name=apr-util
pkg_derivation=chef
pkg_version=1.5.4
pkg_license=('Apache2')
pkg_source=http://www.us.apache.org/dist/apr/${pkg_name}-${pkg_version}.tar.bz2
pkg_shasum=a6cf327189ca0df2fb9d5633d7326c460fe2b61684745fd7963e79a6dd0dc82e
pkg_gpg_key=3853DA6B
pkg_deps=(chef/glibc chef/apr)
pkg_binary_path=(bin)

do_build() {
	./configure --prefix=${pkg_prefix} --with-apr=$(pkg_path_for chef/apr)
	make
        make install
}
