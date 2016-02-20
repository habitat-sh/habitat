pkg_name=binutils
pkg_derivation=chef
pkg_version=2.25.1
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_license=('gpl')
pkg_source=http://ftp.gnu.org/gnu/$pkg_name/${pkg_name}-${pkg_version}.tar.bz2
pkg_shasum=b5b14added7d78a8d1ca70b5cb75fef57ce2197264f4f5835326b0df22ac9f22
pkg_deps=(chef/glibc chef/zlib)
pkg_build_deps=(chef/coreutils chef/diffutils chef/patch chef/make chef/gcc chef/texinfo chef/expect chef/dejagnu)
pkg_binary_path=(bin)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)
pkg_gpg_key=3853DA6B

do_prepare() {
  _verify_tty

  # Add explicit linker instructions as the binutils we are using may have its
  # own dynamic linker defaults.
  dynamic_linker="$(pkg_path_for glibc)/lib/ld-linux-x86-64.so.2"
  LDFLAGS="$LDFLAGS -Wl,-rpath=${LD_RUN_PATH},--enable-new-dtags"
  LDFLAGS="$LDFLAGS -Wl,--dynamic-linker=$dynamic_linker"
  export LDFLAGS
  build_line "Updating LDFLAGS=$LDFLAGS"

  # Don't depend on dynamically linked libgcc, as we don't want it denpending
  # on our /tools install.
  export CFLAGS="$CFLAGS -static-libgcc"
  build_line "Updating CFLAGS=$CFLAGS"

  # TODO: For the wrapper scripts to function correctly, we need the full
  # path to bash. Until a bash plan is created, we're going to wing this...
  bash=/bin/bash

  # Make `--enable-new-dtags` the default so that the linker sets `RUNPATH`
  # instead of `RPATH` in ELF binaries. This is important as `RPATH` is
  # overridden if `LD_LIBRARY_PATH` is set at runtime.
  #
  # Thanks to: https://github.com/NixOS/nixpkgs/blob/2524504/pkgs/development/tools/misc/binutils/new-dtags.patch
  # Thanks to: https://build.opensuse.org/package/view_file?file=ld-dtags.diff&package=binutils&project=devel%3Agcc&srcmd5=011dbdef56800d1cd2fa8c585b3dd7db
  patch -p1 < $PLAN_CONTEXT/new-dtags.patch

  # Since binutils 2.22, DT_NEEDED flags aren't copied for dynamic outputs.
  # That requires upstream changes for things to work. So we can patch it to
  # get the old behaviour fo now.
  #
  # Thanks to: https://github.com/NixOS/nixpkgs/blob/d9f4b0a/pkgs/development/tools/misc/binutils/dtneeded.patch
  patch -p1 < $PLAN_CONTEXT/dt-needed-true.patch

  # # Make binutils output deterministic by default.
  #
  # Thanks to: https://github.com/NixOS/nixpkgs/blob/0889bbe/pkgs/development/tools/misc/binutils/deterministic.patch
  patch -p1 < $PLAN_CONTEXT/more-deterministic-output.patch

  cat $PLAN_CONTEXT/custom-libs.patch \
    | sed -e "s,@dynamic_linker@,$dynamic_linker,g" \
      -e "s,@glibc_lib@,$(pkg_path_for chef/glibc)/lib,g" \
      -e "s,@zlib_lib@,$(pkg_path_for chef/zlib)/lib,g" \
    | patch -p1

  # We don't want to search for libraries in system directories such as `/lib`,
  # `/usr/local/lib`, etc.
  echo 'NATIVE_LIB_DIRS=' >> ld/configure.tgt

  # Use symlinks instead of hard links to save space (otherwise `strip(1)`
  # needs to process each hard link seperately)
  for f in binutils/Makefile.in gas/Makefile.in ld/Makefile.in gold/Makefile.in; do
    sed -i "$f" -e 's|ln |ln -s |'
  done
}

do_build() {
  rm -rf ../${pkg_name}-build
  mkdir ../${pkg_name}-build
  pushd ../${pkg_name}-build > /dev/null
    ../$pkg_dirname/configure \
      --prefix=$pkg_prefix \
      --enable-shared \
      --enable-deterministic-archives \
      --enable-threads \
      --disable-werror

    # Check the environment to make sure all the necessary tools are available
    make configure-host

    make -j$(nproc) tooldir=$pkg_prefix
  popd > /dev/null
}

do_check() {
  pushd ../${pkg_name}-build > /dev/null
    # This testsuite is pretty sensitive to its environment, especially when
    # libraries and headers are being flown in from non-standard locations.
    original_LD_RUN_PATH="$LD_RUN_PATH"
    export LD_LIBRARY_PATH="$LD_RUN_PATH:$(pkg_path_for gcc)/lib"
    unset LD_RUN_PATH

    make check LDFLAGS=""

    unset LD_LIBRARY_PATH
    export LD_RUN_PATH="$original_LD_RUN_PATH"
  popd > /dev/null
}

do_install() {
  pushd ../${pkg_name}-build > /dev/null
    make prefix=$pkg_prefix tooldir=$pkg_prefix install

    # Remove unneeded files
    rm -fv ${pkg_path}/share/man/man1/{dlltool,nlmconv,windres,windmc}*

    # No shared linking to these files outside binutils
    rm -fv ${pkg_path}/lib/lib{bfd,opcodes}.so

    # Wrap key binaries so we can add some arguments and flags to the real
    # underlying binary.
    #
    # Thanks to: https://github.com/NixOS/nixpkgs/blob/master/pkgs/build-support/cc-wrapper/ld-wrapper.sh
    # Thanks to: https://gcc.gnu.org/onlinedocs/gcc/Directory-Options.html
    _wrap_binary ld.bfd
  popd > /dev/null
}

_verify_tty() {
  # verify that PTYs are working properly
  local actual
  local expected='spawn ls'
  local cmd="expect -c 'spawn ls'"
  if actual=$(expect -c "spawn ls" | sed 's/\r$//'); then
    if [[ $expected != $actual ]]; then
      exit_with "Expected out from '$cmd' was: '$expected', actual: '$actual'" 1
    fi
  else
    exit_with "PTYs may not be working properly, aborting" 1
  fi
}

_wrap_binary() {
  local bin="$pkg_path/bin/$1"
  build_line "Adding wrapper $bin to ${bin}.real"
  mv -v "$bin" "${bin}.real"
  sed $PLAN_CONTEXT/ld-wrapper.sh \
    -e "s^@shell@^${bash}^g" \
    -e "s^@dynamic_linker@^${dynamic_linker}^g" \
    -e "s^@program@^${bin}.real^g" \
    > "$bin"
  chmod 755 "$bin"
}


# ----------------------------------------------------------------------------
# **NOTICE:** What follows are implementation details required for building a
# first-pass, "stage1" toolchain and environment. It is only used when running
# in a "stage1" Studio and can be safely ignored by almost everyone. Having
# said that, it performs a vital bootstrapping process and cannot be removed or
# significantly altered. Thank you!
# ----------------------------------------------------------------------------
if [[ "$STUDIO_TYPE" = "stage1" ]]; then
  pkg_build_deps=()
fi
