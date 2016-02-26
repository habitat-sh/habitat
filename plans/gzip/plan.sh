pkg_name=gzip
pkg_origin=chef
pkg_version=1.6
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_license=('gplv3+')
pkg_source=http://ftp.gnu.org/gnu/$pkg_name/${pkg_name}-${pkg_version}.tar.xz
pkg_shasum=37dfed1a485d53212c43b3fa2a7c7952f09bf5cd86e37121c222341ee1b27847
pkg_deps=(chef/glibc chef/less chef/grep)
pkg_build_deps=(chef/coreutils chef/diffutils chef/patch chef/make chef/gcc chef/sed chef/xz)
pkg_binary_path=(bin)
pkg_gpg_key=3853DA6B

do_prepare() {
  do_default_prepare

  build_line "Patching 'zless' with the full path to 'less'"
  sed -i \
    -e "s,less -V,$(pkg_path_for less)/bin/less -V,g" \
    -e "s,exec less,exec $(pkg_path_for less)/bin/less,g" \
    zless.in
}

do_build() {
  ./configure \
    --prefix=$pkg_prefix
  # Prevent a hard dependency on the grep package
  make \
    -j$(nproc) \
    GREP=$(pkg_path_for grep)/bin/grep \
    LESS=$(pkg_path_for less)/bin/less
}

do_check() {
  make check
}


# ----------------------------------------------------------------------------
# **NOTICE:** What follows are implementation details required for building a
# first-pass, "stage1" toolchain and environment. It is only used when running
# in a "stage1" Studio and can be safely ignored by almost everyone. Having
# said that, it performs a vital bootstrapping process and cannot be removed or
# significantly altered. Thank you!
# ----------------------------------------------------------------------------
if [[ "$STUDIO_TYPE" = "stage1" ]]; then
  pkg_build_deps=(chef/gcc chef/coreutils chef/sed chef/diffutils chef/xz)
fi
