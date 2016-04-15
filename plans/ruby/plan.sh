pkg_name=ruby
pkg_origin=core
pkg_version=2.3.0
pkg_license=('ruby')
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_source=https://cache.ruby-lang.org/pub/${pkg_name}/${pkg_name}-${pkg_version}.tar.gz
pkg_filename=${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=ba5ba60e5f1aa21b4ef8e9bf35b9ddb57286cb546aac4b5a28c71f459467e507
pkg_deps=(core/glibc core/ncurses core/zlib core/libedit core/openssl core/libyaml
          core/libiconv core/libffi)
pkg_build_deps=(core/coreutils core/diffutils core/patch core/make core/gcc core/sed)
pkg_lib_dirs=(lib)
pkg_include_dirs=(include)
pkg_bin_dirs=(bin)
pkg_interpreters=(bin/ruby)

do_build() {
    CFLAGS="${CFLAGS} -O3 -g -pipe"
    patch -p1 -i $PLAN_CONTEXT/patches/ruby-2_1_3-no-mkmf.patch

    ./configure --prefix=${pkg_prefix} \
                --with-out-ext=dbm \
                --enable-shared \
                --enable-libedit \
                --disable-install-doc \
                --without-gmp \
                --without-gdbm \
                --with-openssl-dir=$(_resolve_dependency core/openssl) \
                --with-libyaml-dir=$(_resolve_dependency core/libyaml)
    make
}
