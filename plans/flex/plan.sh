pkg_name=flex
pkg_derivation=chef
pkg_version=2.6.0
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_license=('custom')
pkg_source=http://downloads.sourceforge.net/sourceforge/${pkg_name}/${pkg_name}-${pkg_version}.tar.xz
pkg_shasum=d39b15a856906997ced252d76e9bfe2425d7503c6ed811669665627b248e4c73
pkg_deps=(chef/glibc)
pkg_build_deps=(chef/gcc chef/m4 chef/coreutils chef/bison)
pkg_binary_path=(bin)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)
pkg_gpg_key=3853DA6B

do_build() {
  do_default_build

  if [[ -n "$DO_CHECK" ]]; then
    build_line "Running post-compile tests"
    # Set `LDFLAGS` for the c++ test code to find libstdc++
    make check LDFLAGS="$LDFLAGS -lstdc++"
  fi
}

do_install() {
  do_default_install

  # A few programs do not know about `flex` yet and try to run its predecessor,
  # `lex`
  ln -sv flex $pkg_path/bin/lex
}
