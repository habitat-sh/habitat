# shellcheck disable=2034

pkg_name=hab
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
# There is no true equivalent here (yet), so dependency arrays will be empty.
pkg_deps=()
pkg_build_deps=(
	core/tar
	core/coreutils
	core/perl
	core/protobuf
	core/rust/"$(tail -n 1 "$SRC_PATH/../../rust-toolchain"  | cut -d'"' -f 2)"
)
pkg_bin_dirs=(bin)

pkg_version() {
	cat "$SRC_PATH/../../VERSION"
}

do_before() {
	build_line "$PWD"
	do_default_before
	update_pkg_version

	# shellcheck disable=2154
	pkg_filename=${pkg_name}-${pkg_version}.tar.gz
}

do_download() {
	local tar_binary
	tar_binary=$(pkg_path_for tar)/bin/tar

	pushd "$INITIAL_PWD" > /dev/null || exit

	build_line "Creating The source tar file. $pkg_filename in $PWD."
	$tar_binary -czf "$HAB_CACHE_SRC_PATH"/"$pkg_filename" components/ test-services/ Cargo.toml Cargo.lock  || exit

	popd || exit
}

do_verify() {
	return 0
}

do_unpack() {
	local tar_binary
	tar_binary=$(pkg_path_for tar)/bin/tar

	build_line "Unpacking the sources."

	pushd "$HAB_CACHE_SRC_PATH" > /dev/null || exit

	# shellcheck disable=2154
	mkdir "$pkg_dirname"
	tar -C "$pkg_dirname" -xzf "$pkg_filename"

	popd || exit
}

# shellcheck disable=2154
do_prepare() {
  # Can be either `--release` or `--debug` to determine cargo build strategy
  build_type="--release"
  build_line "Building artifacts with \`${build_type#--}' mode"

  export rustc_target="aarch64-apple-darwin"
  build_line "Setting rustc_target=$rustc_target"

  # Used by the `build.rs` program to set the version of the binaries
  export PLAN_VERSION="${pkg_version}/${pkg_release}"
  build_line "Setting PLAN_VERSION=$PLAN_VERSION"

  # Used by Cargo to use a pristine, isolated directory for all compilation
  export CARGO_TARGET_DIR="$HAB_CACHE_SRC_PATH/$pkg_dirname"
  build_line "Setting CARGO_TARGET_DIR=$CARGO_TARGET_DIR"
}

do_build() {
  pushd "$SRC_PATH" > /dev/null || exit
  # strip out mac-bootstrapper which will inject non-ARM binaries
  PATH=${PATH/":/opt/mac-bootstrapper/embedded/bin:"/:} cargo build ${build_type#--debug} --target=$rustc_target --verbose
  popd > /dev/null || exit
}

# Override the do_strip implementation that the x86_64-linux plan.sh includes
#
# TODO(SM): MacOS strip doesn't provide any of --strip-{all,debug,unnecessary}
# that we use in plan-build, so I don't believe we've ever stripped our MacOS 
# binaries.  Running `file $(which hab)` returns `regular file` which would 
# bypass the existing behavior. For now, lets always skip stripping to prevent 
# accidental breakages from changes to plan-build.  
do_strip() {
  build_line "Skipping do_strip on aarch64-darwin"
  return 0
}

do_install() {
	# shellcheck disable=2154
	install -v -D "$CARGO_TARGET_DIR"/"$rustc_target"/release/hab "$pkg_prefix"/bin/hab
}
