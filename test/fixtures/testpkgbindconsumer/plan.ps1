$pkg_name="testpkgbindconsumer"
$pkg_origin="habitat-testing"
$pkg_version="0.1.0"
$pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
$pkg_license=@("Apache-2.0")
$pkg_binds=@{
    "alias" = "setting"
}

function Invoke-Install {
    mkdir "$pkg_prefix\hooks"
    Set-Content -Path "$pkg_prefix\hooks\run" -Value "while (`$true) { Start-Sleep 1 }"
}
