pkg_name=dejagnu
pkg_derivation=chef
pkg_version=1.5.3
pkg_license=('gplv2+')
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_source=http://ftp.gnu.org/gnu/$pkg_name/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=099b8e364ca1d6248f8e1d32168c4b12677abff4253bbbb4a8ac8cdd321e3f19
pkg_deps=(chef/expect)
pkg_build_deps=(chef/gcc chef/sed chef/bison chef/flex chef/grep chef/bash chef/gawk chef/libtool chef/diffutils chef/findutils chef/xz chef/gettext chef/gzip chef/make chef/patch chef/texinfo chef/util-linux)
pkg_binary_path=(bin)
pkg_include_dirs=(include)
pkg_gpg_key=3853DA6B

do_check() {
  # The test-suite needs to have a non-empty stdin, see:
  # http://lists.gnu.org/archive/html/bug-dejagnu/2003-06/msg00002.html
  #
  # Provide `runtest' with a log name, otherwise it tries to run `whoami`,
  # which fails when in a chroot.
  LOGNAME="dejagnu-logger" make check < /dev/zero
}

do_install() {
  do_default_install

  # Set an absolute path `expect` in the `runtest` binary
  sed \
    -e "s,expectbin=expect,expectbin=$(pkg_path_for expect)/bin/expect,g" \
    -i $pkg_path/bin/runtest
}
