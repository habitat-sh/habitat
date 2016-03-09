pkg_name=imagemagick
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_origin=chef
pkg_version=6.9.2-10
pkg_license=('Apache2')
pkg_maintainer="The Bldr Maintainers (bldr@chef.io)"
pkg_source=http://www.imagemagick.org/download/releases/ImageMagick-${pkg_version}.tar.xz
pkg_shasum=da2f6fba43d69f20ddb11783f13f77782b0b57783dde9cda39c9e5e733c2013c
pkg_gpg_key=3853DA6B
pkg_binary_path=(bin)
pkg_deps=(chef/zlib chef/glibc)
pkg_build_deps=(chef/zlib chef/coreutils chef/diffutils chef/patch chef/make chef/gcc chef/sed chef/glibc)
pkg_dirname=ImageMagick-${pkg_version}

do_build() {
  CC="gcc -std=gnu99" ./configure --disable-openmp
  make
}
