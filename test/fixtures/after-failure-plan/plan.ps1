$pkg_name="after_failure_plan"
$pkg_origin="habitat-testing"
$pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
$pkg_version="0.0.0"

function Invoke-Build {
    throw "build is broken"
}

function Invoke-AfterSuccess {
    Write-Host "I am a success"
}

function Invoke-AfterFailure {
    Write-Host "I am a failure"
}
