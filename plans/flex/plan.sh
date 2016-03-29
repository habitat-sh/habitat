pkg_name=flex
pkg_origin=chef
pkg_version=2.6.0
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_license=('custom')
pkg_source=http://downloads.sourceforge.net/sourceforge/${pkg_name}/${pkg_name}-${pkg_version}.tar.xz
pkg_shasum=d39b15a856906997ced252d76e9bfe2425d7503c6ed811669665627b248e4c73
pkg_deps=(chef/glibc)
pkg_build_deps=(chef/coreutils chef/diffutils chef/patch chef/make chef/gcc chef/m4 chef/bison)
pkg_binary_path=(bin)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)
pkg_gpg_key=3853DA6B

do_check() {
  # Set `LDFLAGS` for the c++ test code to find libstdc++
  make check LDFLAGS="$LDFLAGS -lstdc++"
}

do_install() {
  do_default_install

  # A few programs do not know about `flex` yet and try to run its predecessor,
  # `lex`
  ln -sv flex $pkg_prefix/bin/lex
}


# ----------------------------------------------------------------------------
# **NOTICE:** What follows are implementation details required for building a
# first-pass, "stage1" toolchain and environment. It is only used when running
# in a "stage1" Studio and can be safely ignored by almost everyone. Having
# said that, it performs a vital bootstrapping process and cannot be removed or
# significantly altered. Thank you!
# ----------------------------------------------------------------------------
if [[ "$STUDIO_TYPE" = "stage1" ]]; then
  pkg_build_deps=(chef/gcc chef/m4 chef/coreutils chef/bison)
fi
