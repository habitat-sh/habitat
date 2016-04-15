pkg_name=nodejs-tutorial-app
pkg_version=0.1.0
pkg_origin=core
pkg_license=('MIT')
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_source=https://s3-us-west-2.amazonaws.com/${pkg_name}/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=2ad73b78ef5e88e9e916873c0f762bbdf286e4de93e67cf211f9761a2876c7ef
pkg_filename=${pkg_name}-${pkg_version}.tar.gz
pkg_deps=(core/node)
pkg_build_deps=()
pkg_expose=(8080)

do_build () {
  # This installs both npm as well as the nconf module we listed as a dependency in package.json.
  npm install
}

do_install() {
  # Copy our source files from HAB_CACHE_SRC_PATH to the nodejs-tutorial-app
  # package.  This is so that when Habitat calls "npm start" at start-up, we
  # have the source files included in the package.
  cp package.json ${pkg_path}
  cp server.js ${pkg_path}

  # Copy over the nconf module to the package that we installed in do_build().
  mkdir -p ${pkg_prefix}/node_modules/
  cp -vr node_modules/* ${pkg_prefix}/node_modules/
}
