pkg_name=curl
pkg_version=7.47.1
pkg_origin=chef
pkg_license=('curl')
pkg_source=https://curl.haxx.se/download/${pkg_name}-${pkg_version}.tar.gz
pkg_filename=${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=4e9d85028e754048887505a73638bf9b254c39582a191f43c95fe7de8e4d8005
pkg_gpg_key=3853DA6B
pkg_deps=(chef/glibc chef/openssl chef/zlib)
pkg_build_deps=(chef/gcc chef/make chef/coreutils chef/perl)
pkg_lib_dirs=(lib)
pkg_include_dirs=(include)
pkg_bin_dirs=(bin)

do_build() {
    fix_interpreter scripts/zsh.pl chef/perl bin/perl
    ./configure --prefix=${pkg_prefix} \
                --with-ssl=$(pkg_path_for chef/openssl) \
                --with-zlib=$(pkg_path_for chef/zlib) \
                --disable-manual \
                --disable-ldap \
                --disable-ldaps \
                --disable-rtsp \
                --enable-proxy \
                --enable-optimize \
                --disable-dependency-tracking \
                --enable-ipv6 \
                --without-libidn \
                --without-gnutls \
                --without-librtmp
    make
    make install
}
