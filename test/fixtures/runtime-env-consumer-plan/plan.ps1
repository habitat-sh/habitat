$pkg_name="runtime-env-consumer-plan"
$pkg_origin="habitat-testing"
$pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
$pkg_version="0.0.0"
$pkg_build_deps=@("habitat-testing/runtime-env-plan")

function Invoke-Build {
    if($env:SOME_VAR -ne "SOME_VAL") {
        Write-Error "build failed"
    }
}