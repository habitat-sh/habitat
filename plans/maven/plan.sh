pkg_origin=core
pkg_name=maven
pkg_version=3.3.9
pkg_maintainer='The Habitat Maintainers <humans@habitat.sh>'
pkg_license=('Apache-2.0')
pkg_source=http://apache.cs.utah.edu/maven/maven-3/${pkg_version}/source/apache-maven-${pkg_version}-src.tar.gz
pkg_shasum=9150475f509b23518e67a220a9d3a821648ab27550f4ece4d07b92b1fc5611bc
pkg_deps=(core/jdk8)
pkg_build_deps=(core/jdk8 core/ant)
pkg_bin_dirs=(bin)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)

do_build() {
  export JAVA_HOME=$(hab pkg path core/jdk8)
  pushd $HAB_CACHE_SRC_PATH/apache-maven-$pkg_version
  ant -Dmaven.home="${pkg_prefix}"
}


do_install() {
  return 0
}
