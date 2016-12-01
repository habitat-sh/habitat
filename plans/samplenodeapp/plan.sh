pkg_name=samplenodeapp
pkg_version=0.1.0
pkg_origin=chef
pkg_license=('MIT')
pkg_maintainer="The Habitat Maintainers <bldr@chef.io>"
pkg_source=https://s3-us-west-2.amazonaws.com/sample-node-app/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=5d2645001338a65b27d73afa2623a054de42b7e39588102ecf74bc1782378d9d
pkg_filename=${pkg_name}-${pkg_version}.tar.gz
pkg_gpg_key=3853DA6B
pkg_deps=(chef/glibc chef/node)
pkg_expose=(8080)
pkg_build_deps=(chef/node chef/coreutils chef/gcc chef/gcc-libs)

do_build () {
    
  # This installs both npm as well as the nconf module we listed as a dependency in package.json. 
  npm install 
}

do_install() {
    
  # Copy our source files from BLDR_SRC_CACHE to the samplenodeapp package. 
  # This is so that when Habitat calls "npm start" at start-up, we have the source files 
  # included in the package.
  cp package.json ${pkg_path}
  cp server.js ${pkg_path}
  
  # Copy over the nconf module to the package that we installed in do_build().
  mkdir -p ${pkg_path}/node_modules/
  cp -r node_modules/* ${pkg_path}/node_modules/
}
