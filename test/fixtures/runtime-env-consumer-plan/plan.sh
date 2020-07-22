pkg_name="runtime-env-consumer-plan"
pkg_origin="habitat-testing"
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_version="0.0.0"
pkg_build_deps=("habitat-testing/runtime-env-plan")

do_build() {
    if [[ "$SOME_VAR" != "SOME_VAL" ]]; then
        exit 1
    else
        exit 0
    fi
}
do_install() { :; }
