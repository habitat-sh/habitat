pkg_name="broken_after_success_plan"
pkg_origin="habitat-testing"
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_version="0.0.0"

do_build() { :; }
do_install() { :; }

do_after_success() {
    exit 1
}

do_after_failure() {
    echo "I am a failure"
}
