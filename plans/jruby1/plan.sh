pkg_origin=core
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_name=jruby1
pkg_version=1.7.25
pkg_source=https://github.com/jruby/jruby/archive/${pkg_version}.tar.gz
pkg_shasum=4e17872bc38cf6c0ff238a365d2046e36e3149d0d381df2198fd949902602c9c
pkg_license=('EPL 1.0, GPL 2 and LGPL 2.1')
pkg_deps=(core/glibc core/server-jre core/bash)
pkg_build_deps=(core/which core/make core/jdk8 core/coreutils)
pkg_bin_dirs=(bin)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)
pkg_dirname=jruby-${pkg_version}

do_build() {
  export JAVA_HOME=$(pkg_path_for core/jdk8)
  ./mvnw
}

do_install() {
  cp -R * $pkg_prefix/
  for binstub in ${pkg_prefix}/bin/*; do
    [[ -f $binstub ]] && sed -e "s#/usr/bin/env bash#$(pkg_path_for bash)/bin/bash#" -i $binstub
    [[ -f $binstub ]] && sed -e "s#/usr/bin/env jruby#${pkg_prefix}/bin/jruby#" -i $binstub
  done
}
