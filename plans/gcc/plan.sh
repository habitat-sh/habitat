pkg_name=gcc
pkg_distname=$pkg_name
pkg_origin=chef
pkg_version=5.2.0
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_license=('gpl')
pkg_source=http://ftp.gnu.org/gnu/$pkg_distname/${pkg_distname}-${pkg_version}/${pkg_distname}-${pkg_version}.tar.bz2
pkg_shasum=5f835b04b5f7dd4f4d2dc96190ec1621b8d89f2dc6f638f9f8bc1b1014ba8cad
pkg_deps=(chef/glibc chef/zlib chef/gmp chef/mpfr chef/libmpc chef/binutils)
pkg_build_deps=(chef/coreutils chef/diffutils chef/patch chef/make chef/gcc chef/gawk chef/m4 chef/texinfo chef/perl chef/inetutils chef/expect chef/dejagnu)
pkg_bin_dirs=(bin)
pkg_include_dirs=(include)
pkg_lib_dirs=(lib)
pkg_gpg_key=3853DA6B

do_prepare() {
  glibc="$(pkg_path_for glibc)"
  binutils="$(pkg_path_for binutils)"
  headers="$glibc/include"

  # Add explicit linker instructions as the binutils we are using may have its
  # own dynamic linker defaults.
  dynamic_linker="$(pkg_path_for glibc)/lib/ld-linux-x86-64.so.2"
  LDFLAGS="$LDFLAGS -Wl,-rpath=${LD_RUN_PATH},--enable-new-dtags"
  LDFLAGS="$LDFLAGS -Wl,--dynamic-linker=$dynamic_linker"
  build_line "Updating LDFLAGS=$LDFLAGS"

  # Remove glibc include directories from `$CFLAGS` as their contents will be
  # included in the `--with-native-system-header-dir` configure option
  orig_cflags="$CFLAGS"
  CFLAGS=
  for include in $orig_cflags; do
    if ! echo "$include" | grep -q "${glibc}" > /dev/null; then
      CFLAGS="$CFLAGS $include"
    fi
  done
  export CFLAGS
  build_line "Updating CFLAGS=$CFLAGS"

  # Set `CXXFLAGS` for the c++ code
  export CXXFLAGS="$CXXFLAGS $CFLAGS"
  build_line "Setting CXXFLAGS=$CXXFLAGS"

  # Ensure gcc can find the headers for zlib
  export CPATH="$(pkg_path_for zlib)/include"
  build_line "Setting CPATH=$CPATH"

  # Ensure gcc can find the shared libs for zlib
  export LIBRARY_PATH="$(pkg_path_for zlib)/lib"
  build_line "Setting LIBRARY_PATH=$LIBRARY_PATH"

  # TODO: For the wrapper scripts to function correctly, we need the full
  # path to bash. Until a bash plan is created, we're going to wing this...
  bash=/bin/bash

  # Tell gcc not to look under the default `/lib/` and `/usr/lib/` directories
  # for libraries
  #
  # Thanks to: https://github.com/NixOS/nixpkgs/blob/release-15.09/pkgs/development/compilers/gcc/no-sys-dirs.patch
  patch -p1 < $PLAN_CONTEXT/no-sys-dirs.patch

  # Patch the configure script so it finds glibc headers
  #
  # Thanks to: https://github.com/NixOS/nixpkgs/blob/release-15.09/pkgs/development/compilers/gcc/builder.sh
  sed -i \
    -e "s,glibc_header_dir=/usr/include,glibc_header_dir=${headers}," \
    gcc/configure

  # Use the correct path to the dynamic linker instead of the default
  # `lib/ld*.so`
  #
  # Thanks to: https://github.com/NixOS/nixpkgs/blob/release-15.09/pkgs/development/compilers/gcc/5/default.nix
  build_line "Fixing the GLIBC_DYNAMIC_LINKER and UCLIBC_DYNAMIC_LINKER macros"
  for header in "gcc/config/"*-gnu.h "gcc/config/"*"/"*.h; do
    grep -q LIBC_DYNAMIC_LINKER "$header" || continue
    build_line "  Fixing $header"
    sed -i "$header" \
      -e 's|define[[:blank:]]*\([UCG]\+\)LIBC_DYNAMIC_LINKER\([0-9]*\)[[:blank:]]"\([^\"]\+\)"$|define \1LIBC_DYNAMIC_LINKER\2 "'${headers}'\3"|g' \
      -e 's|/lib64/ld-linux-|/lib/ld-linux-|g'
  done

  # Installs x86_64 libraries under `lib/` vs the default `lib64/`
  #
  # Thanks to: https://projects.archlinux.org/svntogit/packages.git/tree/trunk/PKGBUILD?h=packages/gcc
  sed -i '/m64=/s/lib64/lib/' gcc/config/i386/t-linux64

  # Build up the build cflags that will be set for multiple environment
  # variables in the `make` command
  build_cflags="-O2"
  build_cflags="$build_cflags -I${headers}"
  build_cflags="$build_cflags -B${glibc}/lib/"
  build_cflags="$build_cflags -idirafter"
  build_cflags="$build_cflags ${headers}"
  build_cflags="$build_cflags -idirafter"
  build_cflags="$build_cflags ${pkg_prefix}/lib/gcc/*/*/include-fixed"
  build_cflags="$build_cflags -Wl,-L${glibc}/lib"
  build_cflags="$build_cflags -Wl,-rpath"
  build_cflags="$build_cflags -Wl,${glibc}/lib"
  build_cflags="$build_cflags -Wl,-L${glibc}/lib"
  build_cflags="$build_cflags -Wl,-dynamic-linker"
  build_cflags="$build_cflags -Wl,${dynamic_linker}"

  # Build up the target ldflags that will be used in the `make` command
  target_ldflags="-Wl,-L${glibc}/lib"
  target_ldflags="$target_ldflags -Wl,-rpath"
  target_ldflags="$target_ldflags -Wl,${glibc}/lib"
  target_ldflags="$target_ldflags -Wl,-L${glibc}/lib"
  target_ldflags="$target_ldflags -Wl,-dynamic-linker"
  target_ldflags="$target_ldflags -Wl,${dynamic_linker}"
  target_ldflags="$target_ldflags -Wl,-L${glibc}/lib"
  target_ldflags="$target_ldflags -Wl,-rpath"
  target_ldflags="$target_ldflags -Wl,${glibc}/lib"
  target_ldflags="$target_ldflags -Wl,-L${glibc}/lib"
  target_ldflags="$target_ldflags -Wl,-dynamic-linker"
  target_ldflags="$target_ldflags -Wl,${dynamic_linker}"
}

do_build() {
  rm -rf ../${pkg_name}-build
  mkdir ../${pkg_name}-build
  pushd ../${pkg_name}-build > /dev/null
    SED=sed \
    LD=$(pkg_path_for chef/binutils)/bin/ld \
    AS=$(pkg_path_for chef/binutils)/bin/as \
    ../$pkg_dirname/configure \
      --prefix=$pkg_prefix \
      --with-gmp=$(pkg_path_for chef/gmp) \
      --with-mpfr=$(pkg_path_for chef/mpfr) \
      --with-mpc=$(pkg_path_for chef/libmpc) \
      --with-native-system-header-dir=$headers \
      --enable-languages=c,c++ \
      --enable-lto \
      --enable-plugin \
      --enable-shared \
      --enable-threads=posix \
      --enable-install-libiberty \
      --disable-werror \
      --disable-multilib \
      --with-system-zlib \
      --disable-libstdcxx-pch

    # Don't store the configure flags in the resulting executables.
    #
    # Thanks to: https://github.com/NixOS/nixpkgs/blob/release-15.09/pkgs/development/compilers/gcc/builder.sh
    sed -e '/TOPLEVEL_CONFIGURE_ARGUMENTS=/d' -i Makefile

    # CFLAGS_FOR_TARGET are needed for the libstdc++ configure script to find
    # the startfiles.
    # FLAGS_FOR_TARGET are needed for the target libraries to receive the -Bxxx
    # for the startfiles.
    #
    # Thanks to: https://github.com/NixOS/nixpkgs/blob/release-15.09/pkgs/development/compilers/gcc/builder.sh
    make \
      -j$(nproc) \
      NATIVE_SYSTEM_HEADER_DIR=$headers \
      SYSTEM_HEADER_DIR=$headers \
      CFLAGS_FOR_BUILD="$build_cflags" \
      CXXFLAGS_FOR_BUILD="$build_cflags" \
      CFLAGS_FOR_TARGET="$build_cflags" \
      CXXFLAGS_FOR_TARGET="$build_cflags" \
      FLAGS_FOR_TARGET="$build_cflags" \
      LDFLAGS_FOR_BUILD="$build_cflags" \
      LDFLAGS_FOR_TARGET="$target_ldflags" \
      BOOT_CFLAGS="$build_cflags" \
      BOOT_LDFLAGS="$build_cflags" \
      LIMITS_H_TEST=true \
      profiledbootstrap
  popd > /dev/null
}

do_check() {
  pushd ../${pkg_name}-build > /dev/null
    # One set of tests in the GCC test suite is known to exhaust the stack,
    # so increase the stack size prior to running the tests
    ulimit -s 32768

    unset CPATH LIBRARY_PATH
    export LIBRARY_PATH="$LD_RUN_PATH"
    # Do not abort on error as some are "expected"
    make -k check || true
    unset LIBRARY_PATH

    build_line "Displaying Test Summary"
    ../$pkg_dirname/contrib/test_summary
  popd > /dev/null
}

do_install() {
  pushd ../${pkg_name}-build > /dev/null
    # Make 'lib64' a symlink to 'lib'
    mkdir -pv $pkg_prefix/lib
    ln -sv lib $pkg_prefix/lib64

    make install

    # Install PIC version of libiberty which lets Binutils successfully build.
    # As of some point in the near past (2015+ ?), the GCC distribution
    # maintains the libiberty code and not Binutils (they each used to
    # potentially install `libiberty.a` which was confusing as to the "owner").
    #
    # Thanks to: https://projects.archlinux.org/svntogit/packages.git/tree/trunk/PKGBUILD?h=packages/gcc
    install -v -m644 libiberty/pic/libiberty.a $pkg_prefix/lib

    # Install Runtime Library Exception
    install -Dm644 ../$pkg_dirname/COPYING.RUNTIME \
      $pkg_prefix/share/licenses/RUNTIME.LIBRARY.EXCEPTION

    # Replace hard links for x86_64-unknown-linux-gnu etc. with symlinks
    #
    # Thanks to: https://github.com/NixOS/nixpkgs/blob/release-15.09/pkgs/development/compilers/gcc/builder.sh
    for bin in $pkg_prefix/bin/*-gcc*; do
      if cmp -s $pkg_prefix/bin/gcc $bin; then
        ln -sfnv gcc $bin
      fi
    done

    # Replace hard links for x86_64-unknown-linux-g++ etc. with symlinks
    for bin in $pkg_prefix/bin/c++ $pkg_prefix/bin/*-c++* $pkg_prefix/bin/*-g++*; do
      if cmp -s $pkg_prefix/bin/g++ $bin; then
        ln -sfn g++ $bin
      fi
    done

    # Many packages use the name cc to call the C compiler
    ln -sv gcc $pkg_prefix/bin/cc

    # Wrap key binaries so we can add some arguments and flags to the real
    # underlying binary. This should make Plan author's lives a bit easier
    # as they won't have to worry about setting the correct dynamic linker
    # (from glibc) and finding the correct path to the special linker object
    # files such as `crt1.o` and gang.
    #
    # Thanks to: https://github.com/NixOS/nixpkgs/blob/master/pkgs/build-support/cc-wrapper/cc-wrapper.sh
    # Thanks to: https://gcc.gnu.org/onlinedocs/gcc/Directory-Options.html
    wrap_binary gcc
    wrap_binary g++
    wrap_binary cpp
  popd > /dev/null
}

wrap_binary() {
  local bin="$pkg_prefix/bin/$1"
  build_line "Adding wrapper $bin to ${bin}.real"
  mv -v "$bin" "${bin}.real"
  sed $PLAN_CONTEXT/cc-wrapper.sh \
    -e "s^@shell@^${bash}^g" \
    -e "s^@glibc@^${glibc}^g" \
    -e "s^@binutils@^${binutils}^g" \
    -e "s^@gcc@^${pkg_prefix}^g" \
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
  pkg_build_deps=(chef/m4)
fi
