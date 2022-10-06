pkg_name="build-tools-hab-backline"
pkg_origin="core"
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('Apache-2.0')
pkg_build_deps=()

pkg_deps=(
    core/build-tools-hab-plan-build
    core/build-tools-diffutils
    core/build-tools-make
    core/build-tools-ncurses
)

pkg_version() {
    cat "$SRC_PATH/../../VERSION"
}

do_before() {
    do_default_before
    update_pkg_version
}

do_build() {
    return 0
}

do_install() {
    return 0
}
