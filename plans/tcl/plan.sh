pkg_name=tcl
pkg_derivation=chef
pkg_version=8.6.4
pkg_license=('custom')
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_source=http://downloads.sourceforge.net/sourceforge/$pkg_name/${pkg_name}${pkg_version}-src.tar.gz
pkg_shasum=9e6ed94c981c1d0c5f5fefb8112d06c6bf4d050a7327e95e71d417c416519c8d
pkg_dirname=${pkg_name}${pkg_version}
pkg_deps=(chef/glibc chef/zlib)
pkg_build_deps=(chef/gcc chef/coreutils chef/sed chef/bison chef/flex chef/grep chef/bash chef/gawk chef/libtool chef/diffutils chef/findutils chef/xz chef/gettext chef/gzip chef/make chef/patch chef/texinfo chef/util-linux)
pkg_binary_path=(bin)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)
pkg_gpg_key=3853DA6B

do_build() {
  pushd unix > /dev/null
    ./configure \
      --prefix=$pkg_prefix \
      --enable-threads \
      --enable-64bit
    make

    # The Tcl package expects that its source tree is preserved so that
    # packages depending on it for their compilation can utilize it. These sed
    # remove the references to the build directory and replace them with saner
    # system-wide locations.
    #
    # Thanks to: http://www.linuxfromscratch.org/blfs/view/stable/general/tcl.html
    # Thanks to: https://projects.archlinux.org/svntogit/packages.git/tree/trunk/PKGBUILD?h=packages/tcl
    local srcdir=$(abspath ..)
    local tdbcver=tdbc1.0.3
    local itclver=itcl4.0.3
    sed \
      -e "s#$srcdir/unix#$pkg_prefix/lib#" \
      -e "s#$srcdir#$pkg_prefix/include#" \
      -i tclConfig.sh
    sed \
      -e "s#$srcdir/unix/pkgs/$tdbcver#$pkg_prefix/lib/$tdbcver#" \
      -e "s#$srcdir/pkgs/$tdbcver/generic#$pkg_prefix/include#" \
      -e "s#$srcdir/pkgs/$tdbcver/library#$pkg_prefix/lib/tcl${pkg_version%.?}#" \
      -e "s#$srcdir/pkgs/$tdbcver#$pkg_prefix/include#" \
      -i pkgs/$tdbcver/tdbcConfig.sh
    sed \
      -e "s#$srcdir/unix/pkgs/$itclver#$pkg_prefix/lib/$itclver#" \
      -e "s#$srcdir/pkgs/$itclver/generic#$pkg_prefix/include#" \
      -e "s#$srcdir/pkgs/$itclver#$pkg_prefix/include#" \
      -i pkgs/itcl4.0.3/itclConfig.sh
  popd > /dev/null
}

do_install() {
  pushd unix > /dev/null
    make install
    make install-private-headers

    # Many packages expect a file named tclsh, so create a symlink
    ln -sfv tclsh${pkg_version%.?} $pkg_path/bin/tclsh

    chmod -v 755 $pkg_path/lib/libtcl${pkg_version%.?}.so
    ln -sfv libtcl${pkg_version%.?}.so $pkg_path/lib/libtcl.so

    # Install license file
    install -Dm644 ../license.terms ${pkg_path}/share/licenses/LICENSE
  popd > /dev/null
}
