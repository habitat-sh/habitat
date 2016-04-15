pkg_name=apr-util
pkg_origin=core
pkg_version=1.5.4
pkg_license=('Apache2')
pkg_source=http://www.us.apache.org/dist/apr/${pkg_name}-${pkg_version}.tar.bz2
pkg_shasum=a6cf327189ca0df2fb9d5633d7326c460fe2b61684745fd7963e79a6dd0dc82e
pkg_deps=(core/glibc core/apr)
pkg_build_deps=(core/gcc core/make)
pkg_bin_dirs=(bin)

do_build() {
	./configure --prefix=${pkg_prefix} --with-apr=$(pkg_path_for core/apr)
	make
}
