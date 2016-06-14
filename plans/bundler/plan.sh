pkg_name=bundler
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_version=1.11.2
pkg_origin=core
pkg_license=('bundler')
pkg_source=
pkg_filename=nosuchfile.tar.gz
pkg_shasum=
pkg_deps=(core/glibc core/ruby)
pkg_build_deps=(core/ruby)
pkg_lib_dirs=(lib)
pkg_include_dirs=(include)
pkg_bin_dirs=(bin vendor/bundle/bin)

do_install() {
  export GEM_HOME=$pkg_prefix
  export GEM_PATH=$pkg_prefix
  gem install bundler -v ${pkg_version} --no-ri --no-rdoc
}

do_download() {
  return 0
}

do_verify() {
  return 0
}

do_unpack() {
  return 0
}

do_prepare() {
  return 0
}

do_build() {
  return 0
}
