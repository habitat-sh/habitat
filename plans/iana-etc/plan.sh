pkg_name=iana-etc
pkg_derivation=chef
pkg_version=2.30
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_license=('gplv3+')
pkg_source=http://sethwklein.net/${pkg_name}-${pkg_version}.tar.bz2
pkg_shasum=b9a6874fb20012836efef75452ef2acae624022d680feeb1994d73facba3f20d
pkg_deps=()
pkg_build_deps=(chef/coreutils chef/make chef/gawk)
pkg_gpg_key=3853DA6B

do_build() {
  make
}

do_install() {
  make install PREFIX=$pkg_prefix
}


# ----------------------------------------------------------------------------
# **NOTICE:** What follows are implementation details required for building a
# first-pass, "stage1" toolchain and environment. It is only used when running
# in a "stage1" Studio and can be safely ignored by almost everyone. Having
# said that, it performs a vital bootstrapping process and cannot be removed or
# significantly altered. Thank you!
# ----------------------------------------------------------------------------
if [[ "$STUDIO_TYPE" = "stage1" ]]; then
  pkg_build_deps=(chef/coreutils chef/gawk)
fi
