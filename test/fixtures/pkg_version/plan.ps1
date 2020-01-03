$pkg_name="pkg_version"
$pkg_origin="habitat-testing"
$pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"

function Invoke-SetupEnvironment {
    Push-RuntimeEnv "SomeVar" "SomeValue"
}

function pkg_version {
    "5.5.5"
}

function Invoke-Before {
    Invoke-DefaultBefore
    Set-PkgVersion
}
