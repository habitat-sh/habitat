$pkg_name="runtime-env-plan"
$pkg_origin="habitat-testing"
$pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
$pkg_version="0.0.0"

function Invoke-SetupEnvironment {
    Set-RuntimeEnv "SOME_VAR" "SOME_VAL"
}
