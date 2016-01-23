pkg_name=mg
pkg_derivation=chef
pkg_version=20160118
pkg_license=('publicdomain')
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_source=http://homepage.boetes.org/software/$pkg_name/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=26450b2564bec0b0afc465fd24a1917dc31508c5500c3a36823b9c763a2b8636
pkg_deps=(chef/glibc chef/ncurses chef/libbsd)
pkg_build_deps=(chef/gcc chef/coreutils chef/sed chef/bison chef/flex chef/grep chef/bash chef/gawk chef/libtool chef/diffutils chef/findutils chef/xz chef/gettext chef/gzip chef/make chef/patch chef/texinfo chef/util-linux chef/pkg-config chef/clens)
pkg_binary_path=(bin)
pkg_gpg_key=3853DA6B

do_prepare() {
  cat $PLAN_CONTEXT/cleanup.patch \
    | sed \
      -e "s,@prefix@,$pkg_prefix,g" \
      -e "s,@clens_prefix@,$(pkg_path_for clens),g" \
      -e "s,@libbsd_prefix@,$(pkg_path_for libbsd),g" \
    | patch -p1

  export PKG_CONFIG_PATH=$(pkg_path_for libbsd)/lib/pkgconfig
}
do_build() {
  make \
    prefix=$pkg_prefix \
    PKG_CONFIG=pkg-config \
    INSTALL=install \
    STRIP=strip
}

do_check() {
  make check
}

do_install() {
  do_default_install

  # Install license file from README
  install -Dm644 README "$pkg_path/share/licenses/README"
}
