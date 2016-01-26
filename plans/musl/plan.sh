pkg_name=musl
pkg_derivation=chef
pkg_version=1.1.12
pkg_license=('mit')
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_source=http://www.musl-libc.org/releases/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=720b83c7e276b4b679c0bffe9509340d5f81fd601508e607e708177df0d31c0e
pkg_deps=()
pkg_build_deps=(chef/gcc chef/coreutils chef/sed chef/bison chef/flex chef/grep chef/bash chef/gawk chef/libtool chef/diffutils chef/findutils chef/xz chef/gettext chef/gzip chef/make chef/patch chef/texinfo chef/util-linux)
pkg_binary_path=(bin)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)
pkg_gpg_key=3853DA6B

do_build() {
  ./configure \
    --prefix=$pkg_prefix \
    --syslibdir=$pkg_prefix/lib
  make -j$(nproc)
}

do_install() {
  do_default_install

  # Install license
  install -Dm0644 COPYRIGHT $pkg_path/share/licenses/COPYRIGHT
}
