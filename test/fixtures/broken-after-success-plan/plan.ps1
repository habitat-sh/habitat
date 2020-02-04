$pkg_name="broken_after_success_plan"
$pkg_origin="habitat-testing"
$pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
$pkg_version="0.0.0"

function Invoke-AfterSuccess {
    throw "failure comes after success"
}

function Invoke-AfterFailure {
    Write-Host "I am a failure"
}
