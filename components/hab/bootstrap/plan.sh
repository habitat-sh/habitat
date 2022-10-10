# shellcheck disable=2154
native_target="${TARGET_ARCH:-${pkg_target%%-*}}-hab-linux-gnu"

pkg_name="build-tools-hab"
pkg_origin="core"
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('Apache-2.0')
# The result is a portable, static binary in a zero-dependency package.
pkg_deps=(
    core/build-tools-glibc
    core/build-tools-gcc-libs
)

pkg_bin_dirs=(bin)

bin="hab"

pkg_version() {
    cat "$SRC_PATH/../../VERSION"
}

do_begin() {
    path_backup="$PATH"
}

do_before() {
    do_default_before
    update_pkg_version
}

do_prepare() {
    build_type="--release"
    build_line "Building artifacts with \`${build_type#--}' mode"

    export CARGO_TARGET_DIR="$HAB_CACHE_SRC_PATH/$pkg_dirname"

    build_line "Setting CARGO_TARGET_DIR=$CARGO_TARGET_DIR"

    export rustc_target="${TARGET_ARCH:-${pkg_target%%-*}}-unknown-linux-gnu"
    build_line "Setting rustc_target=$rustc_target"

    # Restore the original path so gcc does not interfere with the
    # native C compiler and build process
    PATH=$path_backup
    unset CFLAGS
    unset CPPFLAGS
    unset CXXFLAGS
    unset LD_RUN_PATH
    unset LDFLAGS
}

do_build() {
    pushd "$SRC_PATH" >/dev/null || exit
    cargo build ${build_type#--debug} --target="$rustc_target" --verbose
    popd >/dev/null || exit
}

do_install() {
    case $native_target in
    aarch64-hab-linux-gnu)
        dynamic_linker="$(pkg_path_for build-tools-glibc)/lib/ld-linux-aarch64.so.1"
        ;;
    x86_64-hab-linux-gnu)
        dynamic_linker="$(pkg_path_for build-tools-glibc)/lib/ld-linux-x86-64.so.2"
        ;;
    esac
    install -v -D "$CARGO_TARGET_DIR"/"$rustc_target"/${build_type#--}/$bin "$pkg_prefix"/bin/$bin
    patchelf --set-rpath "$(pkg_path_for build-tools-glibc)/lib:$(pkg_path_for build-tools-gcc-libs)/lib" --set-interpreter "$dynamic_linker" "$pkg_prefix"/bin/$bin
}

do_strip() {
    if [[ "$build_type" != "--debug" ]]; then
        strip --strip-debug "$pkg_prefix"/bin/$bin
    fi
}
