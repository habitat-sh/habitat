source ../gcc/plan.sh

pkg_name=gcc-libs
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"

# The shared libraries only depend on chef/glibc
pkg_deps=(chef/glibc)
# Add the same version of the full gcc package as a build dep
pkg_build_deps=(chef/gcc/$pkg_version chef/patchelf)

# Zero out the bin and include paths, as we're only shipping shared libs
pkg_binary_path=()
pkg_include_dirs=()

pkg_gpg_key=3853DA6B

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
  mkdir -pv $pkg_path/lib
  for lib in "${_gcc_libs[@]}"; do
    cp -av $(pkg_path_for gcc)/lib/${lib}.* $pkg_path/lib/
  done
  rm -fv $pkg_path/lib/*.spec $pkg_path/lib/*.py

  mkdir -pv $pkg_path/share/licenses
  cp -av $(pkg_path_for gcc)/share/licenses/RUNTIME.LIBRARY.EXCEPTION \
    $pkg_path/share/licenses/

  # Due to the copy-from-package trick above, the resulting `RUNPATH` entries
  # have more path entries than are actually being used (for mpfr, libmpc,
  # etc), so we'll use `patchelf` trim these unused path entries for each
  # shared library.
  find $pkg_path/lib -type f -name '*.so.*' \
    -exec patchelf --set-rpath $(pkg_path_for glibc)/lib:$pkg_path/lib {} \;
  find $pkg_path/lib -type f -name '*.so.*' -exec patchelf --shrink-rpath {} \;
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
