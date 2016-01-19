pkg_name=gdbm
pkg_derivation=chef
pkg_version=1.11
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_license=('gplv3+')
pkg_source=http://ftp.gnu.org/gnu/$pkg_name/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=8d912f44f05d0b15a4a5d96a76f852e905d051bb88022fcdfd98b43be093e3c3
pkg_deps=(chef/glibc)
pkg_build_deps=(chef/gcc chef/coreutils chef/sed chef/bison chef/flex chef/grep chef/bash chef/gawk chef/libtool)
pkg_binary_path=(bin)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)
pkg_gpg_key=3853DA6B

do_build() {
  ./configure \
    --prefix=$pkg_prefix \
    --enable-libgdbm-compat
  make

  if [[ -n "$DO_CHECK" ]]; then
    build_line "Running post-compile tests"
    make check
  fi
}

do_install() {
  do_default_install

  # create symlinks for compatibility
  install -dm755 ${pkg_path}/include/gdbm
  ln -sf ../gdbm.h ${pkg_path}/include/gdbm/gdbm.h
  ln -sf ../ndbm.h ${pkg_path}/include/gdbm/ndbm.h
  ln -sf ../dbm.h  ${pkg_path}/include/gdbm/dbm.h
}
