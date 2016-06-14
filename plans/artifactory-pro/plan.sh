pkg_origin=core
pkg_name=artifactory-pro
pkg_version=4.8.1
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=("JFrog Artifactory EULA")
pkg_source=https://dl.bintray.com/jfrog/${pkg_name}/org/artifactory/pro/jfrog-${pkg_name}/${pkg_version}/jfrog-${pkg_name}-${pkg_version}.zip
pkg_shasum=481f755a51faa33492829becb5f00ed3e07a392a0669a02d4d874db13dbcbc3f
pkg_deps=(core/bash core/server-jre)
pkg_build_deps=(core/unzip)
pkg_expose=(8081)

do_build() {
  fix_interpreter "bin/artifactory.sh" core/bash bin/bash
  return 0
}

do_install() {
  build_line "Copying files from $PWD"
  mkdir -p $pkg_prefix
  cp -R * $pkg_prefix/
}
