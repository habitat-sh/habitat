pkg_origin=core
pkg_name=docutils
pkg_version='0.12'
pkg_maintainer='The Habitat Maintainers <humans@habitat.sh>'
pkg_license=(
  'GPL-3.0',
  'BSD-2-Clause-FreeBSD',
  'Python-2.0',
  'Docutils Public Domain Dedication'
)
pkg_source=http://heanet.dl.sourceforge.net/project/docutils/docutils/0.12/docutils-0.12.tar.gz
pkg_shasum=c7db717810ab6965f66c8cf0398a98c9d8df982da39b4cd7f162911eb89596fa
pkg_deps=()
pkg_build_deps=(
  core/make
  core/gcc
  core/python2
)
pkg_bin_dirs=(bin)
pkg_lib_dirs=(lib)

do_build() {
  python setup.py install --prefix=$pkg_prefix
}

do_install() {
  python install.py
}
