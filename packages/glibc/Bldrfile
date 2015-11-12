pkg_name=glibc
pkg_derivation=chef
pkg_version=2.19
pkg_license=('GPLv2' 'LGPLv2.1')
pkg_source=http://ftp.gnu.org/gnu/libc/${pkg_name}-${pkg_version}.tar.bz2
pkg_filename=${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=d17a843419530035e73f65e8ecf75e3bb7ea9548d3469bd67f3f769a03ee39c1
pkg_lib_dirs=(lib lib/x86_64-linux-gnu lib64)
pkg_bin_dirs=(usr/bin sbin /usr/sbin)
pkg_include_dirs=(include)
pkg_gpg_key=3853DA6B

download() {
	return 0
}

verify() {
	return 0
}

unpack() {
	return 0
}

prepare() {
	pkg_dirname="$pkg_name-$pkg_version"
	mkdir -p $BLDR_SRC_CACHE/$pkg_dirname
}

build() {
	dpkg -L libc6 > $BLDR_SRC_CACHE/$pkg_dirname/files
	dpkg -L libc-bin >> $BLDR_SRC_CACHE/$pkg_dirname/files
}

install() {
	mkdir -p $pkg_path
	rsync -vaP --no-dirs --files-from=$BLDR_SRC_CACHE/$pkg_dirname/files / $pkg_path
}

