pkg_name=test_build_with_secrets
pkg_origin=core
pkg_version="0.1.0"
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=("Apache-2.0")

do_build() {
    set -u
    # The build will fail if the FOO environment variable is not set.
    echo "The secret is $FOO"
    set +u
}

do_install() {
    return 0
}
