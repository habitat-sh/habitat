pkg_name=docker
pkg_origin=chef
pkg_version=1.10.1
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_license=('Apache-2')
pkg_source=https://get.docker.com/builds/Linux/x86_64/docker-1.10.1.tgz
pkg_shasum=2287bc5cbcd1cdad77f1c0c70c2b5b15f1d9c010900c3ffab059fb46fe81d141
pkg_deps=()
pkg_build_deps=()
pkg_binary_path=(bin)
pkg_gpg_key=3853DA6B

do_unpack() {
  pushd $BLDR_SRC_CACHE
  mkdir -p $pkg_dirname
  tar xf $pkg_filename -C $BLDR_SRC_CACHE/$pkg_dirname
}

do_build() {
  return 0
}

do_install() {
  mkdir -p $pkg_prefix/bin
  cp $BLDR_SRC_CACHE/$pkg_dirname/usr/local/bin/docker $pkg_prefix/bin
  chmod a+x $pkg_prefix/bin/docker
}
