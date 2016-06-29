pkg_name=aws-cli
pkg_origin=core
pkg_version=1.10.43
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('Apache-2.0')
pkg_description="The AWS Command Line Interface (CLI) is a unified tool to manage your AWS services. With just one tool to download and configure, you can control multiple AWS services from the command line and automate them through scripts."
pkg_upstream_url=https://aws.amazon.com/cli/
pkg_source=nosuchfile.tgz
pkg_build_deps=(core/python)
pkg_deps=(
  core/groff
  core/python
)
pkg_bin_dirs=(
  $(pkg_path_for core/groff)/bin
  bin
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
  pyvenv $pkg_prefix
  source $pkg_prefix/bin/activate
}

do_build() {
  return 0
}

do_install() {
  pip install "awscli==$pkg_version"
  # Delete all the virtualenv binaries and leave the aws ones
  find $pkg_prefix/bin -type f ! -name aws* -delete
}
