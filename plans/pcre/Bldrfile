pkg_name=pcre
pkg_derivation=chef
pkg_version=8.37
pkg_license=('bsd')
pkg_source=https://ftp.csx.cam.ac.uk/pub/software/programming/${pkg_name}/${pkg_name}-${pkg_version}.tar.bz2
pkg_shasum=51679ea8006ce31379fb0860e46dd86665d864b5020fc9cd19e71260eef4789d
pkg_gpg_key=3853DA6B
pkg_deps=(chef/glibc chef/libedit chef/ncurses chef/zlib chef/bzip2)
pkg_lib_dirs=(lib)
pkg_binary_path=(bin)
pkg_include_dirs=(include)

build() {
  ./configure --prefix=$pkg_prefix \
    --enable-unicode-properties \
    --enable-pcre16 \
    --enable-pcre32 \
    --enable-jit \
    --enable-pcregrep-libz \
    --enable-pcregrep-libbz2 \
    --enable-pcretest-libedit
  make
}
