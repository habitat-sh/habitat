pkg_name=groff
pkg_origin=core
pkg_version=1.22.3
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('GPL-3.0')
pkg_description="Groff (GNU troff) is a typesetting system that reads plain text mixed with formatting commands and produces formatted output. Output may be PostScript or PDF, html, or ASCII/UTF8 for display at the terminal. Formatting commands may be either low-level typesetting requests (“primitives”) or macros from a supplied set. Users may also write their own macros. All three may be combined."
pkg_upstream_url=https://www.gnu.org/software/groff/
pkg_source=http://ftp.gnu.org/gnu/groff/groff-1.22.3.tar.gz
pkg_shasum=3a48a9d6c97750bfbd535feeb5be0111db6406ddb7bb79fc680809cda6d828a5
pkg_bin_dirs=(bin)
pkg_build_deps=(
  core/gcc
  core/gcc-libs
  core/make
  core/perl
)
pkg_deps=(
  core/gcc-libs
)
