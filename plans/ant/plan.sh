pkg_origin=core
pkg_name=ant
pkg_version=1.9.7
pkg_maintainer='The Habitat Maintainers <humans@habitat.sh>'
pkg_license=('Apache-2.0')
pkg_source=https://github.com/apache/ant/archive/rel/$pkg_version.tar.gz
pkg_shasum=53dbfb13e0e9bf1b34bcbfdf807c1b9df1b89362b176e1e58150906f518b2c88
pkg_deps=(core/jdk8)
pkg_build_deps=(core/jdk8)
pkg_bin_dirs=(bin)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)

do_build() {
  export JAVA_HOME=$(hab pkg path core/jdk8)
  pushd $HAB_CACHE_SRC_PATH/$pkg_name-rel-$pkg_version
  sh ./build.sh -Ddist.dir=$pkg_prefix dist
}

do_install() {
  return 0
}
