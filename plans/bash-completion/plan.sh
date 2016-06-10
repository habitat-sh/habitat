pkg_name=bash-completion
pkg_origin=core
pkg_version=2.3
pkg_license=('GPLv2')
pkg_maintainer="The Habitat Contributors <humans@habitat.sh>"
pkg_source=https://github.com/scop/bash-completion/releases/download/${pkg_version}/bash-completion-${pkg_version}.tar.xz
pkg_shasum=b2e081af317f3da4fff3a332bfdbebeb5514ebc6c2d2a9cf781180acab15e8e9
pkg_bin_dirs=(bin)
pkg_build_deps=(core/make core/gcc core/autoconf core/automake)
pkg_deps=(core/glibc)

