pkg_origin=core
pkg_name=acbuild
pkg_version=0.3.0
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('Apache-2')
pkg_source=https://github.com/appc/acbuild/releases/download/v${pkg_version}/${pkg_name}.tar.gz
pkg_shasum=da9c90712642d1e540bdb60765d760a459969b603d2604ab1e90b2689a9c3c0b
pkg_deps=(core/gnupg core/glibc)
pkg_build_deps=(core/patchelf)
pkg_bin_dirs=(bin)

do_build() {
  return 0
}

do_install() {
  install -v -D $HAB_CACHE_SRC_PATH/acbuild $pkg_prefix/bin/acbuild

  patchelf \
      --interpreter "$(pkg_path_for glibc)/lib/ld-linux-x86-64.so.2" \
      --set-rpath "$LD_RUN_PATH" \
      "$pkg_prefix/bin/acbuild"
}
