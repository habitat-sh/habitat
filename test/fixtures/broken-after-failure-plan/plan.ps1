$pkg_name="broken_after_failure_plan"
$pkg_origin="habitat-testing"
$pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
$pkg_version="0.0.0"

function Invoke-Build {
    throw "build is broken"
}

function Invoke-AfterSuccess {
    throw "I am a success"
}

function Invoke-AfterFailure {
    throw "failure after failure"
}
