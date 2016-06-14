source ../gcc/plan.sh

pkg_name=gcc-libs
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"

# The shared libraries only depend on core/glibc
pkg_deps=(core/glibc)
# Add the same version of the full gcc package as a build dep
pkg_build_deps=(core/gcc/$pkg_version core/patchelf)

# Zero out the bin and include paths, as we're only shipping shared libs
pkg_bin_dirs=()
pkg_include_dirs=()


# The list of GCC libraries to copy
_gcc_libs=(
  libasan
  libatomic
  libcilkrts
  libgcc_s
  libgomp-plugin-host_nonshm
  libgomp
  libitm
  liblsan
  libquadmath
  libstdc++
  libtsan
  libubsan
  libvtv
)

do_install() {
  mkdir -pv $pkg_prefix/lib
  for lib in "${_gcc_libs[@]}"; do
    cp -av $(pkg_path_for gcc)/lib/${lib}.* $pkg_prefix/lib/
  done
  rm -fv $pkg_prefix/lib/*.spec $pkg_prefix/lib/*.py

  mkdir -pv $pkg_prefix/share/licenses
  cp -av $(pkg_path_for gcc)/share/licenses/RUNTIME.LIBRARY.EXCEPTION \
    $pkg_prefix/share/licenses/

  # Due to the copy-from-package trick above, the resulting `RUNPATH` entries
  # have more path entries than are actually being used (for mpfr, libmpc,
  # etc), so we'll use `patchelf` trim these unused path entries for each
  # shared library.
  find $pkg_prefix/lib -type f -name '*.so.*' \
    -exec patchelf --set-rpath $(pkg_path_for glibc)/lib:$pkg_prefix/lib {} \;
  find $pkg_prefix/lib -type f -name '*.so.*' -exec patchelf --shrink-rpath {} \;
}

# Turn the remaining default phases into no-ops

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

# We will rely on tests from `gcc`, so skip them here
unset -f do_check
