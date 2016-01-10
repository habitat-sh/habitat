source ../gcc/plan.sh

pkg_name=gcc-libs
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_dirname=${pkg_distname}-${pkg_version}

# Add the same version of the full gcc package as a build dep
pkg_build_deps+=(chef/gcc/$pkg_version chef/patchelf)

# Zero out the bin and include paths, as we're only shipping shared libs
pkg_binary_path=()
pkg_include_dirs=()

pkg_gpg_key=3853DA6B

# We will rely on tests from `gcc`, so skip them here
unset -f do_check

do_install() {
  triplet=x86_64-unknown-linux-gnu

  pushd ../${pkg_name}-build > /dev/null
    make -C $triplet/libgcc install-shared
    rm -rf ${pkg_path}/lib/gcc

    for lib in libatomic \
               libcilkrts \
               libgomp \
               libitm \
               libquadmath \
               libsanitizer/{a,l,ub}san \
               libstdc++-v3/src \
               libvtv \
               libsanitizer/tsan; do
      make -C $triplet/$lib install-toolexeclibLTLIBRARIES
    done

    make -C $triplet/libstdc++-v3/po install

    for lib in libgomp \
               libitm \
               libquadmath; do
      make -C $triplet/$lib install-info
    done

    # Install Runtime Library Exception
    install -Dm644 ../$pkg_dirname/COPYING.RUNTIME \
      $pkg_path/share/licenses/RUNTIME.LIBRARY.EXCEPTION

    # As we're building a full gcc and we're only interested in the shared
    # libraries, we're going to trim the build dependencies to glibc and
    # add the others to the build dependency list. Note that this is at
    # least mildy evil and probably not something a normal Plan author
    # would want to do.
    #
    # To be clear: DO NOT USE THIS PATTERN
    orig_pkg_deps_resolved=(${pkg_deps_resolved[@]})
    pkg_deps_resolved=()
    for dep in "${orig_pkg_deps_resolved[@]}"; do
      if echo "$dep" | egrep -q "/chef/glibc/" > /dev/null; then
        pkg_deps_resolved+=($dep)
      else
        pkg_build_deps_resolved+=($dep)
      fi
    done
    orig_pkg_tdeps_resolved=(${pkg_tdeps_resolved[@]})
    pkg_tdeps_resolved=()
    for dep in "${orig_pkg_tdeps_resolved[@]}"; do
      if echo "$dep" | egrep -q "/chef/glibc/" > /dev/null; then
        pkg_tdeps_resolved+=($dep)
      else
        pkg_build_tdeps_resolved+=($dep)
      fi
    done
    pkg_build_tdeps_resolved=(
      $(printf '%s\n' "${pkg_build_tdeps_resolved[@]}" | sort | uniq)
    )

    build_line "Updated dependency list is: (${pkg_deps_resolved[@]})"

    # Due to the build/run package tricks above, the resulting `RUNPATH` has
    # more path entries than are actually being used (for mpfr, libmpc, etc),
    # so we'll use `patchelf` trim these unused path entries for each shared
    # library
    find $pkg_path/lib \
      -type f \
      -name '*.so.*' \
      -exec patchelf --shrink-rpath {} \;
  popd > /dev/null
}
