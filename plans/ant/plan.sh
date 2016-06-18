pkg_origin=core
pkg_name=ant
pkg_version=1.9.7
pkg_maintainer='The Habitat Maintainers <humans@habitat.sh>'
pkg_license=('Apache 2')
pkg_distname=$pkg_name
pkg_source=http://apache.osuosl.org//ant/binaries/apache-ant-${pkg_version}-bin.tar.gz
pkg_shasum=1d0b808fe82cce9bcc167f8dbb7c7e89c1d7f7534c0d9c64bf615ec7c3e6bfe5
pkg_deps=( core/coreutils core/jdk8 core/server-jre )
pkg_build_deps=( core/jdk8 core/server-jre )
pkg_bin_dirs=(bin)
do_build() {
  mkdir -p $pkg_prefix/
  cp -a $PLAN_CONTEXT/apache-ant-${pkg_version}/. $pkg_prefix/
  export JAVA_HOME=$(hab pkg path core/server-jre)
  fix_interpreter ${pkg_prefix}/bin/ant core/coreutils bin/env
}

do_install() {
  export JAVA_HOME=$(hab pkg path core/server-jre)
}
