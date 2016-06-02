pkg_origin=core
pkg_name=haproxy
pkg_version=1.6.5
pkg_maintainer='The Habitat Maintainers <humans@habitat.sh>'
pkg_license=('GPL-2.0', 'LGPL-2.1')
pkg_source=http://www.haproxy.org/download/1.6/src/haproxy-1.6.5.tar.gz
pkg_shasum=c4b3fb938874abbbbd52782087117cc2590263af78fdce86d64e4a11acfe85de
pkg_service_run='bin/haproxy -f config/haproxy.conf -db'
pkg_expose=(8080)
pkg_deps=(core/zlib core/pcre core/openssl)
pkg_build_deps=(
  core/coreutils
  core/gcc
  core/pcre
  core/make
  core/openssl
  core/zlib
)

pkg_bin_dirs=(bin)

do_build() {
  make USE_PCRE=1 \
       USE_PCRE_JIT=1 \
       CPU=native \
       TARGET=linux2628 \
       USE_OPENSSL=1 \
       USE_ZLIB=1 \
       ADDINC="$CFLAGS" \
       ADDLIB="$LDFLAGS"
}

do_install() {
  mkdir -p $pkg_prefix/bin
  cp haproxy $pkg_prefix/bin
}
