pkg_name=patchelf
pkg_derivation=chef
pkg_version=0.8
pkg_maintainer="The Bldr Maintainers <bldr@chef.io>"
pkg_license=('gplv3')
pkg_source=http://releases.nixos.org/$pkg_name/${pkg_name}-$pkg_version/${pkg_name}-${pkg_version}.tar.gz
pkg_shasum=14af06a2da688d577d64ff8dac065bb8903bbffbe01d30c62df7af9bf4ce72fe
pkg_binary_path=(bin)
pkg_build_deps=(chef/binutils chef/gcc)
pkg_gpg_key=3853DA6B

if [[ -n "$BOOTSTRAP_TOOLS" ]]; then
  # Waiting on chef/gcc-libs to link libgcc and libstdc++, but because we need
  # this package to prepare chef/gcc-libs, we'll do the cheap version first
  # that relies on the bootstrapping tools' version of these shared libraries
  pkg_deps=(chef/glibc)
  build_line "Using bootstrap tools from $BOOTSTRAP_TOOLS for libgcc and libstdc++"
else
  pkg_deps=(chef/glibc chef/gcc-libs)
fi

do_prepare() {
  # If chef/gcc-libs is not yet a dependency (i.e. when building from
  # bootstrapping tools), add `/usr/lib` to the `$LD_RUN_PATH` for using the
  # libgcc and libstdc++ from the bootstrapping tools
  if [[ -n "$BOOTSTRAP_TOOLS" ]]; then
    LD_RUN_PATH="$LD_RUN_PATH:/usr/lib"
    build_line "Updating LD_RUN_PATH=$LD_RUN_PATH"
  fi
}
