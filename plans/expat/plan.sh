pkg_name=expat
pkg_derivation=chef
pkg_version=2.1.0
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_license=('mit')
pkg_source=http://downloads.sourceforge.net/sourceforge/expat/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=823705472f816df21c8f6aa026dd162b280806838bb55b3432b0fb1fcca7eb86
pkg_deps=(chef/glibc chef/gcc-libs)
pkg_build_deps=(chef/gcc chef/coreutils chef/sed chef/bison chef/flex chef/grep chef/bash chef/gawk chef/libtool)
pkg_binary_path=(bin)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)
pkg_gpg_key=3853DA6B

do_prepare() {
  do_default_prepare

  # Patch for CVE-2015-1283
  #
  # Thanks to: https://github.com/NixOS/nixpkgs/blob/release-15.09/pkgs/development/libraries/expat/default.nix
  patch -p1 -i $PLAN_CONTEXT/CVE-2015-1283.patch
}

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

  # Install license file
  install -Dm644 COPYING "$pkgdir/share/licenses/COPYING"
}
