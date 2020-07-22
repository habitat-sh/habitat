pkg_name="runtime-env-plan"
pkg_origin="habitat-testing"
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_version="0.0.0"

do_setup_environment() {
    set_runtime_env "SOME_VAR" "SOME_VAL"
}
do_build() { :; }
do_install() { :; }
