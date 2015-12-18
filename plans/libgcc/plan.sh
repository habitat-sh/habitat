pkg_name=libgcc
pkg_derivation=chef
pkg_version=4.9.1
pkg_license=('LGPLv2.1')
pkg_source=http://ftp.gnu.org/gnu/libc/${pkg_name}-${pkg_version}.tar.bz2
pkg_filename=${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=d17a843419530035e73f65e8ecf75e3bb7ea9548d3469bd67f3f769a03ee39c1
pkg_lib_dirs=(lib/x86_64-linux-gnu)
pkg_gpg_key=3853DA6B

do_download() {
	return 0
}

do_verify() {
	return 0
}

do_unpack() {
	return 0
}

do_prepare() {
	pkg_dirname="$pkg_name-$pkg_version"
	mkdir -p $BLDR_SRC_CACHE/$pkg_dirname
}

do_build() {
	dpkg -L libgcc1:amd64 > $BLDR_SRC_CACHE/$pkg_dirname/files
}

do_install() {
	mkdir -p $pkg_path
	rsync -vaP --no-dirs --files-from=$BLDR_SRC_CACHE/$pkg_dirname/files / $pkg_path
}


