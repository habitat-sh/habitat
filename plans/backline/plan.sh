pkg_name=backline
pkg_origin=chef
pkg_version=0.1.0
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_license=('apachev2')
pkg_source=nosuchfile.tar.gz
pkg_build_deps=()
pkg_gpg_key=3853DA6B

pkg_deps=(
  chef/hab-plan-build
  chef/diffutils
  chef/less
  chef/make
  chef/mg
  chef/util-linux
  chef/vim
)

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

do_install() {
  return 0
}
