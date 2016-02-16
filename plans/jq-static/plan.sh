pkg_name=jq-static
pkg_distname=jq
pkg_derivation=chef
pkg_version=1.5
pkg_license=('mit')
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_source=https://github.com/stedolan/$pkg_distname/releases/download/${pkg_distname}-${pkg_version}/jq-linux64
pkg_shasum=c6b3a7d7d3e7b70c6f51b706a3b90bd01833846c54d32ca32f0027f00226ff6d
pkg_deps=()
pkg_build_deps=(chef/coreutils chef/wget)
pkg_binary_path=(bin)
pkg_gpg_key=3853DA6B

do_unpack() {
  return 0
}

do_build() {
  return 0
}

do_install() {
  install -D $BLDR_SRC_CACHE/$pkg_filename $pkg_path/bin/jq
}
