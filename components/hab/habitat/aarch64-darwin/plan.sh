# shellcheck disable=2154

pkg_name=hab
pkg_origin=chef
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('Apache-2.0')

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
    "$tar_binary" -czf "$HAB_CACHE_SRC_PATH"/"$pkg_filename" components/ test-services/ Cargo.toml Cargo.lock  || exit

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
    "$tar_binary" -C "$pkg_dirname" -xzf "$pkg_filename"

    popd || exit
}

# shellcheck disable=2154
do_prepare() {
    local protoc

    export CARGO_HOME="$HAB_CACHE_SRC_PATH/$pkg_dirname/.cargo"
    build_line "Setting CARGO_HOME=$CARGO_HOME"

    export CARGO_TARGET_DIR="$HAB_CACHE_SRC_PATH/$pkg_dirname/target"
    build_line "Setting CARGO_TARGET_DIR=$CARGO_TARGET_DIR"

    export rustc_target="${TARGET_ARCH:-${pkg_target%%-*}}-apple-darwin"
    build_line "Building for target $rustc_target"

    # Used by the `build.rs` program to set the version of the binaries
    export PLAN_VERSION="${pkg_version}/${pkg_release}"
    build_line "Setting PLAN_VERSION=$PLAN_VERSION"

    # Used to set the active package target for the binaries at build time
    export PLAN_PACKAGE_TARGET="$pkg_target"
    build_line "Setting PLAN_PACKAGE_TARGET=$PLAN_PACKAGE_TARGET"

    # Prost (our Rust protobuf library) embeds a `protoc` binary, but
    # it's dynamically linked, which means it won't work in a
    # Studio. Prost does allow us to override that, though, so we can
    # just use our Habitat package by setting these two environment
    # variables.
    protoc="$(pkg_path_for protobuf)"
    export PROTOC="${protoc}/bin/protoc"
    build_line "Setting PROTOC=$PROTOC"

    export PROTOC_INCLUDE="${protoc}/include"
    build_line "Setting PROTOC_INCLUDE=$PROTOC_INCLUDE"

    # Can be either `--release` or `--debug` to determine cargo build strategy
    build_type="--release"
    build_line "Building artifacts with \`${build_type#--}' mode"

    build_line "Setting RUSTFLAGS=$RUSTFLAGS"
}

do_build() {
    pushd "$HAB_CACHE_SRC_PATH/$pkg_dirname" > /dev/null || exit
    cargo build "$build_type" --target="$rustc_target" --verbose --bin hab
    popd >/dev/null || exit
}

do_strip() {
  build_line "Skipping do_strip on aarch64-darwin"
  return 0
}

# shellcheck disable=2154
do_install() {
    install -v -D "$CARGO_TARGET_DIR"/"$rustc_target"/release/hab "$pkg_prefix"/bin/hab
    install -v -D "$SRC_PATH/../../NOTICES.txt" "$pkg_prefix"/bin/NOTICES.txt
}
