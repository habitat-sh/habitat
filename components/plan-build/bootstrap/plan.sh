# shellcheck disable=2034
pkg_name="build-tools-hab-plan-build"
pkg_origin=core
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('Apache-2.0')
pkg_bin_dirs=(bin)

pkg_deps=(
    core/build-tools-bash
    core/build-tools-coreutils
    core/build-tools-file
    core/build-tools-findutils
    core/build-tools-gawk
    core/build-tools-grep
    core/build-tools-gzip
    core/build-tools-hab
    core/build-tools-sed
    core/build-tools-tar
    core/build-tools-xz
    core/build-tools-wget
)

program="hab-plan-build"

pkg_version() {
    cat "$SRC_PATH/../../VERSION"
}

do_before() {
    do_default_before
    update_pkg_version
}

do_build() {
    cp -v "$SRC_PATH"/bin/${program}.sh "$CACHE_PATH/$program"

    # Use the bash from our dependency list as the shebang. Also, embed the
    # release version of the program.
    # shellcheck disable=2154
    sed \
        -e "s,^HAB_PLAN_BUILD=0\.0\.1\$,HAB_PLAN_BUILD=$pkg_version/$pkg_release," \
        -e "s,^pkg_target='@@pkg_target@@'\$,pkg_target='$pkg_target'," \
        -i "$CACHE_PATH/$program"
}

do_check() {
    bats test
}

do_install() {
    # shellcheck disable=2154
    install -D "$CACHE_PATH/$program" "$pkg_prefix"/bin/$program
    install -D "$SRC_PATH"/bin/shared.bash "$pkg_prefix"/bin/
    install -D "$SRC_PATH"/bin/public.bash "$pkg_prefix"/bin/
    install -D "$SRC_PATH"/bin/environment.bash "$pkg_prefix"/bin/
}
