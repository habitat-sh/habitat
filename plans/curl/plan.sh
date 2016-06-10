pkg_name=curl
pkg_origin=core
pkg_version=7.47.1
pkg_maintainer="The Habitat Contributors <humans@habitat.sh>"
pkg_license=('curl')
pkg_source=https://curl.haxx.se/download/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=4e9d85028e754048887505a73638bf9b254c39582a191f43c95fe7de8e4d8005
pkg_deps=(core/glibc core/openssl core/zlib)
pkg_build_deps=(core/gcc core/make core/coreutils core/perl)
pkg_bin_dirs=(bin)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)

do_prepare() {
  # Patch the zsh-generating program to use our perl at build time
  sed -i "s,/usr/bin/perl,$(pkg_path_for perl)/bin/perl,g" scripts/zsh.pl
}

do_build() {
  ./configure --prefix=${pkg_prefix} \
              --with-ssl=$(pkg_path_for openssl) \
              --with-zlib=$(pkg_path_for zlib) \
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
}
