pkg_name=coreutils
pkg_derivation=chef
pkg_version=8.24
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_license=('gplv3')
pkg_source=http://ftp.gnu.org/gnu/$pkg_name/${pkg_name}-${pkg_version}.tar.xz
pkg_shasum=a2d75286a4b9ef3a13039c2da3868a61be4ee9f17d8ae380a35a97e506972170
pkg_deps=(chef/glibc chef/acl chef/attr chef/gmp chef/libcap)
pkg_build_deps=(chef/binutils chef/gcc chef/m4)
pkg_binary_path=(bin)
pkg_gpg_key=3853DA6B

do_build() {
  # The `FORCE_` variable allows the software to compile with the root user,
  # and the `--enable-no-install-program` flag skips installation of binaries
  # that are provided by other pacakges.
  FORCE_UNSAFE_CONFIGURE=1 ./configure \
    --prefix=$pkg_prefix \
    --enable-no-install-program=kill,uptime
  make
}

do_check() {
  make NON_ROOT_USERNAME=nobody check-root
  make RUN_EXPENSIVE_TESTS=yes check
}
