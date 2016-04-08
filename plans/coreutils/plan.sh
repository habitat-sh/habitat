pkg_name=coreutils
pkg_distname=$pkg_name
pkg_origin=chef
pkg_version=8.24
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('gplv3')
pkg_source=http://ftp.gnu.org/gnu/$pkg_distname/${pkg_distname}-${pkg_version}.tar.xz
pkg_shasum=a2d75286a4b9ef3a13039c2da3868a61be4ee9f17d8ae380a35a97e506972170
pkg_deps=(chef/glibc chef/acl chef/attr chef/gmp chef/libcap)
pkg_build_deps=(chef/coreutils chef/diffutils chef/patch chef/make chef/gcc chef/m4 chef/perl)
pkg_bin_dirs=(bin)
pkg_gpg_key=3853DA6B
pkg_interpreters=(bin/env)

do_build() {
  # The `FORCE_` variable allows the software to compile with the root user,
  # and the `--enable-no-install-program` flag skips installation of binaries
  # that are provided by other pacakges.
  FORCE_UNSAFE_CONFIGURE=1 ./configure \
    --prefix=$pkg_prefix \
    --enable-no-install-program=kill,uptime
  make
}

do_check() {
  make NON_ROOT_USERNAME=nobody check-root
  make RUN_EXPENSIVE_TESTS=yes check
}


# ----------------------------------------------------------------------------
# **NOTICE:** What follows are implementation details required for building a
# first-pass, "stage1" toolchain and environment. It is only used when running
# in a "stage1" Studio and can be safely ignored by almost everyone. Having
# said that, it performs a vital bootstrapping process and cannot be removed or
# significantly altered. Thank you!
# ----------------------------------------------------------------------------
if [[ "$STUDIO_TYPE" = "stage1" ]]; then
  pkg_build_deps=(chef/gcc chef/m4)
fi
