pkg_name=bash
pkg_distname=$pkg_name
pkg_derivation=chef
_base_version=4.3
pkg_version=${_base_version}.42
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_license=('gplv3+')
_url_base=http://ftp.gnu.org/gnu/$pkg_distname
pkg_source=$_url_base/${pkg_distname}-${_base_version}.tar.gz
pkg_dirname=${pkg_distname}-$_base_version
pkg_shasum=afc687a28e0e24dc21b988fa159ff9dbcf6b7caa92ade8645cc6d5605cd024d4
pkg_deps=(chef/glibc chef/ncurses chef/readline)
pkg_build_deps=(chef/gcc chef/grep)
pkg_binary_path=(bin)
pkg_gpg_key=3853DA6B

do_begin() {
  # The maintainer of Bash only releases these patches to fix serious issues,
  # so any new official patches will be part of this build, which will be
  # reflected in the "tiny" or "patch" number of the version coordinate. In other
  # words, given 6 patches, the version of this Bash package would be
  # `MAJOR.MINOR.6`.

  # Source a file containing an array of patch URLs and an array of patch file
  # shasums
  source $PLAN_CONTEXT/bash-patches.sh
}

do_download() {
  do_default_download

  # Download all patch files, providing the corresponding shasums so we can
  # skip re-downloading if already present and verified
  for i in $(seq 0 $((${#_patch_files[@]} - 1))); do
    p="${_patch_files[$i]}"
    download_file $p $(basename $p) ${_patch_shasums[$i]}
  done; unset i p
}

do_verify() {
  do_default_verify

  # Verify all patch files against their shasums
  for i in $(seq 0 $((${#_patch_files[@]} - 1))); do
    verify_file $(basename ${_patch_files[$i]}) ${_patch_shasums[$i]}
  done; unset i
}

do_prepare() {
  do_default_prepare

  # Apply all patch files to the extracted source
  for p in "${_patch_files[@]}"; do
    build_line "Applying patch $(basename $p)"
    patch -p0 -i $BLDR_SRC_CACHE/$(basename $p)
  done
}

do_build() {
  ./configure \
    --prefix=$pkg_prefix \
    --with-curses \
    --enable-readline \
    --without-bash-malloc \
    --with-installed-readline=$(pkg_path_for chef/readline)
  make
}

do_check() {
  make tests
}

do_install() {
  do_default_install

  # Add an `sh` which symlinks to `bash`
  ln -sv bash $pkg_path/bin/sh
}
