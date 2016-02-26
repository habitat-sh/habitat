pkg_name=bc
pkg_origin=chef
pkg_version=1.06.95
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_license=('gplv3+')
pkg_source=http://alpha.gnu.org/gnu/$pkg_name/${pkg_name}-${pkg_version}.tar.bz2
pkg_shasum=7ee4abbcfac03d8a6e1a8a3440558a3d239d6b858585063e745c760957725ecc
pkg_deps=(chef/glibc chef/readline)
pkg_build_deps=(chef/coreutils chef/diffutils chef/patch chef/make chef/gcc chef/texinfo)
pkg_binary_path=(bin)
pkg_gpg_key=3853DA6B

do_prepare() {
  do_default_prepare

  # Fix a memory leak.
  #
  # Thanks to: https://projects.archlinux.org/svntogit/packages.git/tree/trunk/bc-1.06.95-void_uninitialized.patch?h=packages/bc
  patch -p0 -i $PLAN_CONTEXT/memory-leak.patch
}

do_build() {
  ./configure \
    --prefix=$pkg_prefix \
    --with-readline
  make
}

do_check() {
  echo "quit" | ./bc/bc -l Test/checklib.b
}


# ----------------------------------------------------------------------------
# **NOTICE:** What follows are implementation details required for building a
# first-pass, "stage1" toolchain and environment. It is only used when running
# in a "stage1" Studio and can be safely ignored by almost everyone. Having
# said that, it performs a vital bootstrapping process and cannot be removed or
# significantly altered. Thank you!
# ----------------------------------------------------------------------------
if [[ "$STUDIO_TYPE" = "stage1" ]]; then
  pkg_build_deps=(chef/gcc chef/coreutils)
fi
