$pkg_name="testpkgstophook"
$pkg_origin="habitat-testing"
$pkg_version="0.1.0"
$pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
$pkg_license=@("Apache-2.0")

function Invoke-Install {
    mkdir "$pkg_prefix\hooks"
    Set-Content -Path "$pkg_prefix\hooks\run" -Value "try { while (`$true) { Start-Sleep 1 }} finally { 'run hook is terminating' | Out-File -path `$env:temp/testpkgstophook.out}"
    Set-Content -Path "$pkg_prefix\hooks\post-stop" -Value "'post-stop hook has fired' | Out-File -path `$env:temp/testpkgstophook.out -Append"
}
