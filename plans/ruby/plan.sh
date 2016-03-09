pkg_name=ruby
pkg_origin=chef
pkg_version=2.3.0
pkg_license=('ruby')
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_source=https://cache.ruby-lang.org/pub/${pkg_name}/${pkg_name}-${pkg_version}.tar.gz
pkg_filename=${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=ba5ba60e5f1aa21b4ef8e9bf35b9ddb57286cb546aac4b5a28c71f459467e507
pkg_gpg_key=3853DA6B
pkg_deps=(chef/glibc chef/ncurses chef/zlib chef/libedit chef/openssl chef/libyaml
          chef/libiconv chef/libffi)
pkg_build_deps=(chef/coreutils chef/diffutils chef/patch chef/make chef/gcc chef/sed)
pkg_lib_dirs=(lib)
pkg_include_dirs=(include)
pkg_binary_path=(bin)
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
                --with-openssl-dir=$(_resolve_dependency chef/openssl) \
                --with-libyaml-dir=$(_resolve_dependency chef/libyaml)
    make
}
