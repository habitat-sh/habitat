pkg_name=ttyrec
pkg_origin=core
pkg_version=1.0.8
pkg_maintainer="The Habitat Contributors <humans@habitat.sh>"
pkg_license=('bsd')
pkg_source=http://0xcc.net/$pkg_name/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=ef5e9bf276b65bb831f9c2554cd8784bd5b4ee65353808f82b7e2aef851587ec
pkg_deps=(core/glibc)
pkg_build_deps=(core/coreutils core/patch core/make core/gcc)
pkg_bin_dirs=(bin)

do_prepare() {
  # Apply third party patch, originally designed for RHEL5
  patch -p1 -i $PLAN_CONTEXT/ttyrec-1.0.8.RHEL5.patch
}

do_build() {
  make
}

do_install() {
  for bin in ttyplay ttyrec ttytime; do
    install -v -D $bin $pkg_prefix/bin/$bin
  done
}
