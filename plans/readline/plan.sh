pkg_name=readline
pkg_origin=chef
_base_version=6.3
pkg_version=${_base_version}.8
pkg_license=('gplv3+')
_url_base=http://ftp.gnu.org/gnu/$pkg_name
pkg_source=$_url_base/${pkg_name}-${_base_version}.tar.gz
pkg_dirname=${pkg_name}-$_base_version
pkg_shasum=56ba6071b9462f980c5a72ab0023893b65ba6debb4eeb475d7a563dc65cafd43
pkg_deps=(chef/glibc chef/ncurses)
pkg_build_deps=(chef/coreutils chef/diffutils chef/patch chef/make chef/gcc chef/bison chef/grep)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)
pkg_gpg_key=3853DA6B

# The maintainer of Readline only releases these patches to fix serious issues,
# so any new official patches will be part of this build, which will be
# reflected in the "tiny" or "patch" number of the version coordinate. In other
# words, given 6 patches, the version of this Readline package would be
# `MAJOR.MINOR.6`.

# Source a file containing an array of patch URLs and an array of patch file
# shasums
source $PLAN_CONTEXT/readline-patches.sh

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

  # This patch is to make sure that `libncurses' is among the `NEEDED'
  # dependencies of `libreadline.so' and `libhistory.so'. Failing to do that,
  # applications linking against Readline are forced to explicitly link against
  # libncurses as well; in addition, this trick doesn't work when using GNU
  # ld's `--as-needed'.
  #
  # Thanks to:
  # https://github.com/NixOS/nixpkgs/blob/release-15.09/pkgs/development/libraries/readline/link-against-ncurses.patch
  build_line "Applying patch link-against-ncurses.patch"
  patch -p1 -i $PLAN_CONTEXT/link-against-ncurses.patch
}

do_install() {
  do_default_install

  # An empty `bin/` directory gets made, which we don't need and is confusing
  rm -rf $pkg_prefix/bin
}


# ----------------------------------------------------------------------------
# **NOTICE:** What follows are implementation details required for building a
# first-pass, "stage1" toolchain and environment. It is only used when running
# in a "stage1" Studio and can be safely ignored by almost everyone. Having
# said that, it performs a vital bootstrapping process and cannot be removed or
# significantly altered. Thank you!
# ----------------------------------------------------------------------------
if [[ "$STUDIO_TYPE" = "stage1" ]]; then
  pkg_build_deps=(chef/gcc chef/bison chef/grep)
fi
