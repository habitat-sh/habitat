pkg_name=libmpc
pkg_distname=mpc
pkg_derivation=chef
pkg_version=1.0.3
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_license=('lgpl')
pkg_source=http://www.multiprecision.org/mpc/download/${pkg_distname}-${pkg_version}.tar.gz
pkg_shasum=617decc6ea09889fb08ede330917a00b16809b8db88c29c31bfbb49cbf88ecc3
pkg_build_deps=(chef/binutils)
pkg_deps=(chef/glibc chef/gmp chef/mpfr)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)
pkg_gpg_key=3853DA6B
pkg_dirname=${pkg_distname}-${pkg_version}

do_prepare() {
  do_default_prepare

  LDFLAGS="$LDFLAGS -Wl,-rpath=${LD_RUN_PATH},--enable-new-dtags"
  build_line "Updating LDFLAGS=$LDFLAGS"
}

do_build() {
  ./configure --prefix=$pkg_prefix
  make

  if [ -n "${DO_CHECK}" ]; then
    make check
  fi
}
