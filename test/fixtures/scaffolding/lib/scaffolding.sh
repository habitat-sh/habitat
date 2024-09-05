scaffolding_load() {
    pkg_deps=("$HAB_ORIGIN/minimal_package" "${pkg_deps[@]}")
    pkg_build_deps=("$HAB_ORIGIN/dep-pkg-1" "${pkg_build_deps[@]}")
}