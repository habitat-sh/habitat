pkg_name=dejagnu
pkg_derivation=chef
pkg_version=1.5.3
pkg_license=('gplv2+')
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_source=http://ftp.gnu.org/gnu/$pkg_name/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=099b8e364ca1d6248f8e1d32168c4b12677abff4253bbbb4a8ac8cdd321e3f19
pkg_deps=(chef/expect)
pkg_build_deps=(chef/coreutils chef/diffutils chef/patch chef/make chef/gcc chef/sed)
pkg_binary_path=(bin)
pkg_include_dirs=(include)
pkg_gpg_key=3853DA6B

do_check() {
  # The test-suite needs to have a non-empty stdin, see:
  # http://lists.gnu.org/archive/html/bug-dejagnu/2003-06/msg00002.html
  #
  # Provide `runtest' with a log name, otherwise it tries to run `whoami`,
  # which fails when in a chroot.
  LOGNAME="dejagnu-logger" make check \
    LD_LIBRARY_PATH="$(pkg_path_for gcc)/lib" < /dev/zero
}

do_install() {
  do_default_install

  # Set an absolute path `expect` in the `runtest` binary
  sed \
    -e "s,expectbin=expect,expectbin=$(pkg_path_for expect)/bin/expect,g" \
    -i $pkg_path/bin/runtest
}


# ----------------------------------------------------------------------------
# **NOTICE:** What follows are implementation details required for building a
# first-pass, "stage1" toolchain and environment. It is only used when running
# in a "stage1" Studio and can be safely ignored by almost everyone. Having
# said that, it performs a vital bootstrapping process and cannot be removed or
# significantly altered. Thank you!
# ----------------------------------------------------------------------------
if [[ "$STUDIO_TYPE" = "stage1" ]]; then
  pkg_build_deps=(chef/gcc chef/coreutils chef/sed chef/diffutils chef/make chef/patch)
fi
