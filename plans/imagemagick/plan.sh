pkg_name=imagemagick
pkg_maintainer="The Habitat Contributors <humans@habitat.sh>"
pkg_origin=core
pkg_version=6.9.2-10
pkg_license=('Apache2')
pkg_maintainer="The Habitat Contributors <humans@habitat.sh>"
pkg_source=http://www.imagemagick.org/download/releases/ImageMagick-${pkg_version}.tar.xz
pkg_shasum=da2f6fba43d69f20ddb11783f13f77782b0b57783dde9cda39c9e5e733c2013c
pkg_bin_dirs=(bin)
pkg_deps=(core/zlib core/glibc)
pkg_build_deps=(core/zlib core/coreutils core/diffutils core/patch core/make core/gcc core/sed core/glibc)
pkg_dirname=ImageMagick-${pkg_version}

do_build() {
  CC="gcc -std=gnu99" ./configure --disable-openmp
  make
}
