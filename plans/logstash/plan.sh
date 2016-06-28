pkg_origin=core
pkg_name=logstash
pkg_version=2.3.3
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=(Apache-2.0)
pkg_source=https://download.elastic.co/${pkg_name}/${pkg_name}/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=51a20fbfe2aa0c5ea49ceda8278a4667289fd1871cf7be4ba1c32bd6cbc71d74
pkg_deps=(core/bash core/server-jre core/jruby1)
pkg_build_deps=(core/bash)
pkg_bin_dirs=(bin)
pkg_lib_dirs=(lib)

do_build() {
  return 0
}

do_install() {
  mkdir -p $pkg_prefix
  cp -r * $pkg_prefix
  rm -rf $pkg_prefix/vendor/jruby
  fix_interpreter "${pkg_prefix}/bin/*" core/bash bin/sh
}
