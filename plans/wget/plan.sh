pkg_name=wget
pkg_distname=$pkg_name
pkg_origin=chef
pkg_version=1.16.3
pkg_license=('gplv3+')
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_source=http://ftp.gnu.org/gnu/$pkg_distname/${pkg_distname}-${pkg_version}.tar.xz
pkg_shasum=67f7b7b0f5c14db633e3b18f53172786c001e153d545cfc85d82759c5c2ffb37
pkg_deps=(chef/glibc chef/libidn chef/zlib chef/openssl chef/cacerts)
pkg_build_deps=(chef/coreutils chef/diffutils chef/patch chef/make chef/gcc chef/sed chef/grep chef/pkg-config)
pkg_bin_dirs=(bin)
pkg_gpg_key=3853DA6B

do_prepare() {
  _wget_common_prepare
}

do_build() {
  ./configure \
    --prefix=$pkg_prefix \
    --with-ssl=openssl \
    --without-libuuid
  make
}

do_install() {
  do_default_install

  cat <<EOF >> $pkg_prefix/etc/wgetrc

# Default root CA certs location
ca_certificate=$(pkg_path_for cacerts)/ssl/certs/cacert.pem
EOF
}

_wget_common_prepare() {
  PKG_CONFIG_PATH="$(pkg_path_for openssl)/lib/pkgconfig"
  PKG_CONFIG_PATH="${PKG_CONFIG_PATH}:$(pkg_path_for zlib)/lib/pkgconfig"
  export PKG_CONFIG_PATH
  build_line "Setting PKG_CONFIG_PATH=$PKG_CONFIG_PATH"

  # Purge the codebase (mostly tests & build Perl scripts) of the hardcoded
  # reliance on `/usr/bin/env`.
  grep -lr '/usr/bin/env' . | while read f; do
    sed -e "s,/usr/bin/env,$(pkg_path_for coreutils)/bin/env,g" -i "$f"
  done
}


# ----------------------------------------------------------------------------
# **NOTICE:** What follows are implementation details required for building a
# first-pass, "stage1" toolchain and environment. It is only used when running
# in a "stage1" Studio and can be safely ignored by almost everyone. Having
# said that, it performs a vital bootstrapping process and cannot be removed or
# significantly altered. Thank you!
# ----------------------------------------------------------------------------
if [[ "$STUDIO_TYPE" = "stage1" ]]; then
  pkg_build_deps=(chef/gcc chef/pkg-config chef/coreutils chef/sed chef/grep chef/diffutils chef/make chef/patch)
fi
