pkg_name=expect
pkg_derivation=chef
pkg_version=5.45
pkg_license=('custom')
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_source=http://downloads.sourceforge.net/project/$pkg_name/Expect/${pkg_version}/${pkg_name}${pkg_version}.tar.gz
pkg_shasum=b28dca90428a3b30e650525cdc16255d76bb6ccd65d448be53e620d95d5cc040
pkg_dirname=${pkg_name}${pkg_version}
pkg_deps=(chef/glibc chef/tcl chef/coreutils)
pkg_build_deps=(chef/gcc chef/sed chef/bison chef/flex chef/grep chef/bash chef/gawk chef/libtool chef/diffutils chef/findutils chef/xz chef/gettext chef/gzip chef/make chef/patch chef/texinfo chef/util-linux)
pkg_binary_path=(bin)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)
pkg_gpg_key=3853DA6B

do_prepare() {
  do_default_prepare

  # Make the path to `stty` absolute from `chef/coreutils`
  sed -i "s,/bin/stty,$(pkg_path_for coreutils)/bin/stty,g" configure
}

do_build() {
  ./configure \
    --prefix=$pkg_prefix \
    --exec-prefix=$pkg_prefix \
    --with-tcl=$(pkg_path_for tcl)/lib \
    --with-tclinclude=$(pkg_path_for tcl)/include
  make
}

do_check() {
  make test
}

do_install() {
  do_default_install

  # Add an absolute path to `tclsh` in each script binary
  find $pkg_prefix/bin \
    -type f \
    -exec sed -e "s,exec tclsh,exec $(pkg_path_for tcl)/bin/tclsh,g" -i {} \;
}
