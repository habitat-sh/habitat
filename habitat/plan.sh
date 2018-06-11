pkg_name=habitat
pkg_origin=core
pkg_version=$(cat "$PLAN_CONTEXT/../VERSION")
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('Apache-2.0')
pkg_deps=(
  core/busybox-static core/glibc core/gcc-libs core/libarchive core/libsodium core/openssl
	core/zeromq
)
pkg_build_deps=(
  core/musl core/zlib-musl core/xz-musl core/bzip2-musl core/libarchive-musl core/openssl-musl
	core/libsodium-musl core/cacerts core/coreutils core/rust core/gcc core/raml2html core/tar
	core/wget
)
pkg_bin_dirs=(bin)

rustc_target_ctl="x86_64-unknown-linux-musl"
rustc_target_supervisor="x86_64-unknown-linux-gnu"

do_prepare() {
  do_default_prepare

  # Can be either `--release` or `--debug` to determine cargo build strategy
  build_type="--release"
  build_line "Building artifacts with \`${build_type#--}' mode"

  # Used by the `build.rs` program to set the version of the binaries
  export PLAN_VERSION="${pkg_version}/${pkg_release}"
  build_line "Setting PLAN_VERSION=$PLAN_VERSION"

  if [ -z "$HAB_CARGO_TARGET_DIR" ]; then
    # Used by Cargo to use a pristine, isolated directory for all compilation
    export CARGO_TARGET_DIR="$HAB_CACHE_SRC_PATH/$pkg_dirname"
  else
    export CARGO_TARGET_DIR="$HAB_CARGO_TARGET_DIR"
  fi
  build_line "Setting CARGO_TARGET_DIR=$CARGO_TARGET_DIR"
}

do_build() {
	_build_supervisor
	_build_ctl
	_build_studio
}

do_install() {
	_install_supervisor
	_install_ctl
	_install_studio
}

do_strip() {
  if [[ "$build_type" != "--debug" ]]; then
    do_default_strip
  fi
}

_build_ctl() {
  export rustc_target=$rustc_target_ctl
  build_line "Setting rustc_target=$rustc_target"

  la_ldflags="-L$(pkg_path_for zlib-musl)/lib -lz"
  la_ldflags="$la_ldflags -L$(pkg_path_for xz-musl)/lib -llzma"
  la_ldflags="$la_ldflags -L$(pkg_path_for bzip2-musl)/lib -lbz2"
  la_ldflags="$la_ldflags -L$(pkg_path_for openssl-musl)/lib -lssl -lcrypto"

  export LIBARCHIVE_LIB_DIR=$(pkg_path_for libarchive-musl)/lib
  export LIBARCHIVE_INCLUDE_DIR=$(pkg_path_for libarchive-musl)/include
  export LIBARCHIVE_LDFLAGS="$la_ldflags"
  export LIBARCHIVE_STATIC=true
  export OPENSSL_LIB_DIR=$(pkg_path_for openssl-musl)/lib
  export OPENSSL_INCLUDE_DIR=$(pkg_path_for openssl-musl)/include
  export OPENSSL_STATIC=true
  export SODIUM_LIB_DIR=$(pkg_path_for libsodium-musl)/lib
  export SODIUM_STATIC=true

  # Used to find libgcc_s.so.1 when compiling `build.rs` in dependencies. Since
  # this used only at build time, we will use the version found in the gcc
  # package proper--it won't find its way into the final binaries.
  export LD_LIBRARY_PATH=$(pkg_path_for gcc)/lib
  build_line "Setting LD_LIBRARY_PATH=$LD_LIBRARY_PATH"

	build_line "Building Habitat CTL"
  pushd "$PLAN_CONTEXT/../components/hab" > /dev/null
  cargo build ${build_type#--debug} --target=$rustc_target --verbose
  popd > /dev/null
}

_build_supervisor() {
  export rustc_target=$rustc_target_supervisor
  build_line "Setting rustc_target=$rustc_target"

  export LIBARCHIVE_LIB_DIR=$(pkg_path_for libarchive)/lib
  export LIBARCHIVE_INCLUDE_DIR=$(pkg_path_for libarchive)/include
  export OPENSSL_LIB_DIR=$(pkg_path_for openssl)/lib
  export OPENSSL_INCLUDE_DIR=$(pkg_path_for openssl)/include
  export SODIUM_LIB_DIR=$(pkg_path_for libsodium)/lib
  export LIBZMQ_PREFIX=$(pkg_path_for zeromq)
  export LIBRARY_PATH=$LIBZMQ_PREFIX/lib

	# TODO (CM, FN): This is not needed to build the supervisor,
	# strictly speaking, but is instead a work-around for how we are
	# currently building packages in Travis; we hypothesize that the
	# build.rs program for habitat_http_client, built during a static
	# hab package build, is being inadvertently used here. Without gcc
	# libs on the LD_LIBRARY_PATH, the program can't find
	# libgcc_s.so.1. This is merely a bandaid until we can overhaul our
	# CI pipeline properly.
	#
	# Used to find libgcc_s.so.1 when compiling `build.rs` in dependencies. Since
	# this used only at build time, we will use the version found in the gcc
	# package proper--it won't find its way into the final binaries.
	export LD_LIBRARY_PATH=$(pkg_path_for gcc)/lib
	build_line "Setting LD_LIBRARY_PATH=$LD_LIBRARY_PATH"

	build_line "Building Habitat Supervisor"
	pushd "$PLAN_CONTEXT/../components/sup" > /dev/null
  cargo build ${build_type#--debug} --target=$rustc_target --verbose --no-default-features \
    --features apidocs
  popd > /dev/null
}

_build_studio() {
  cp -v $PLAN_CONTEXT/../components/studio/bin/hab-studio.sh hab-studio
  cp -v $PLAN_CONTEXT/../components/studio/libexec/hab-studio-profile.sh .
  cp -v $PLAN_CONTEXT/../components/studio/libexec/hab-studio-type-*.sh .

  # Embed the release version and author information of the program.
  sed \
    -e "s,@author@,$pkg_maintainer,g" \
    -e "s,@version@,$pkg_version/$pkg_release,g" \
    -i hab-studio
}

_install_ctl() {
  install -v -D \
		$CARGO_TARGET_DIR/$rustc_target_ctl/${build_type#--}/hab \
    $pkg_prefix/bin/$bin
}

_install_supervisor() {
  install -v -D \
		$CARGO_TARGET_DIR/$rustc_target_supervisor/${build_type#--}/hab-sup \
    $pkg_prefix/bin/$bin
}

_install_studio() {
  install -v -D hab-studio $pkg_prefix/bin/hab-studio
  install -v -D hab-studio-profile.sh $pkg_prefix/libexec/hab-studio-profile.sh
  for f in `ls hab-studio-type-*.sh`; do
    install -v -D $f $pkg_prefix/libexec/$f
  done

  lbb="$pkg_prefix/libexec/busybox"

  # Install a copy of a statically built busybox under `libexec/`
  install -v -D $(pkg_path_for busybox-static)/bin/busybox $lbb
  ln -sv $pkg_prefix/bin/hab $pkg_prefix/libexec/hab
}

