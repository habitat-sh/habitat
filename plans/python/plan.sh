pkg_name=python
pkg_version=3.5.2
pkg_origin=core
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('Python-2.0')
pkg_description="Python is a programming language that lets you work quickly
and integrate systems more effectively."
pkg_dirname=Python-${pkg_version}
pkg_source=https://www.python.org/ftp/python/${pkg_version}/${pkg_dirname}.tgz
pkg_filename=${pkg_dirname}.tgz
pkg_shasum=1524b840e42cf3b909e8f8df67c1724012c7dc7f9d076d4feef2d3eff031e8a0
pkg_deps=(
  core/bzip2
  core/coreutils
  core/gcc-libs
  core/glibc
  core/make
  core/ncurses
  core/openssl
  core/readline
  core/sqlite
  core/zlib
)
pkg_build_deps=(
  core/gcc
  core/linux-headers
  core/sqlite
)
pkg_lib_dirs=(lib)
pkg_bin_dirs=(bin)
pkg_include_dirs=(include Include)
pkg_interpreters=(bin/python bin/python3 bin/python3.5)

do_prepare() {
  sed -i.bak 's/#zlib/zlib/' Modules/Setup.dist
  sed -i -re "/(SSL=|_ssl|-DUSE_SSL|-lssl).*/ s|^#||" Modules/Setup.dist
}

do_build() {
  export CPPFLAGS=$CFLAGS
  export LD_LIBRARY_PATH=$(pkg_path_for gcc)/lib
  ./configure --prefix=${pkg_prefix} \
    --enable-shared
  make
}

do_check() {
  make test
}
