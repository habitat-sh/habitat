pkg_name=vim
pkg_derivation=chef
pkg_version=7.4.1089
pkg_license=('vim')
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_source=http://github.com/$pkg_name/$pkg_name/archive/v${pkg_version}.tar.gz
pkg_shasum=e52f7653a36b690441b47a273b1db72f0eb1e5f6729af25110a84088ca73e872
pkg_deps=(chef/glibc chef/acl chef/ncurses)
pkg_build_deps=(chef/gcc chef/coreutils chef/sed chef/bison chef/flex chef/grep chef/bash chef/gawk chef/libtool chef/diffutils chef/findutils chef/xz chef/gettext chef/gzip chef/make chef/patch chef/texinfo chef/util-linux chef/autoconf chef/pkg-config chef/wget)
pkg_binary_path=(bin)
pkg_gpg_key=3853DA6B

do_prepare() {
  pushd src > /dev/null
    autoconf
  popd > /dev/null

  export CPPFLAGS="$CPPFLAGS $CFLAGS"
  build_line "Setting CPPFLAGS=$CPPFLAGS"
}

do_build() {
  ./configure \
    --prefix=$pkg_prefix \
    --with-compiledby="bldr, vim release $pkg_version" \
    --with-features=huge \
    --enable-acl \
    --with-x=no \
    --disable-gui \
    --enable-multibyte
  make
}

do_check() {
  make test
}

do_install() {
  do_default_install

  # Install license file
  install -Dm644 runtime/doc/uganda.txt $pkg_path/share/licenses/license.txt
}
