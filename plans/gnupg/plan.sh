pkg_name=gnupg
pkg_distname=$pkg_name
pkg_origin=core
pkg_version=1.4.20
pkg_license=('gplv3+')
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_source=ftp://ftp.gnupg.org/gcrypt/${pkg_distname}/${pkg_distname}-${pkg_version}.tar.bz2
pkg_shasum=04988b1030fa28ddf961ca8ff6f0f8984e0cddcb1eb02859d5d8fe0fe237edcc
pkg_deps=(core/glibc core/zlib core/bzip2 core/readline)
pkg_build_deps=(core/coreutils core/diffutils core/patch core/make core/gcc core/sed core/findutils)
pkg_bin_dirs=(bin)

do_build() {
  ./configure \
    --prefix=${pkg_prefix} \
    --sbindir=$pkg_prefix/bin
  make
}

do_check() {
  find tests -type f \
    | xargs sed -e "s,/bin/pwd,$(pkg_path_for coreutils)/bin/pwd,g" -i

  make check
}


# ----------------------------------------------------------------------------
# **NOTICE:** What follows are implementation details required for building a
# first-pass, "stage1" toolchain and environment. It is only used when running
# in a "stage1" Studio and can be safely ignored by almost everyone. Having
# said that, it performs a vital bootstrapping process and cannot be removed or
# significantly altered. Thank you!
# ----------------------------------------------------------------------------
if [[ "$STUDIO_TYPE" = "stage1" ]]; then
  pkg_build_deps=(core/gcc core/coreutils core/sed core/diffutils core/findutils core/make core/patch)
fi
