$pkg_name="hab-studio"
$pkg_origin="core"
$pkg_version=Get-Content "$PLAN_CONTEXT/../../../VERSION"
$pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
$pkg_license=@("Apache-2.0")
$pkg_build_deps=@("core/powershell", "core/hab", "core/hab-plan-build-ps1", "core/7zip")
$pkg_bin_dirs=@("bin")

function Invoke-Build {
  Get-Content "$PLAN_CONTEXT/../bin/hab-studio.ps1" | % {
    $_.Replace("@author@", $pkg_maintainer).Replace("@version@", $pkg_version)
  } | Add-Content -Path hab-studio.ps1
}

function Invoke-Install {
  mkdir "$pkg_prefix/bin/powershell" | Out-Null
  mkdir "$pkg_prefix/bin/hab" | Out-Null
  mkdir "$pkg_prefix/bin/7zip" | Out-Null

  Copy-Item hab-studio.ps1 "$pkg_prefix/bin/hab-studio.ps1"
  Copy-Item $PLAN_CONTEXT/../bin/hab-studio.bat "$pkg_prefix/bin/hab-studio.bat"
  Copy-Item $PLAN_CONTEXT/../bin/SupervisorBootstrapper.cs "$pkg_prefix/bin/SupervisorBootstrapper.cs"

  Copy-Item "$(Get-HabPackagePath powershell)/bin/*" "$pkg_prefix/bin/powershell" -Recurse
  Copy-Item "$(Get-HabPackagePath hab)/bin/*" "$pkg_prefix/bin/hab"
  Copy-Item "$(Get-HabPackagePath 7zip)/bin/*" "$pkg_prefix/bin/7zip"
  Copy-Item "$(Get-HabPackagePath hab-plan-build-ps1)/bin/*" "$pkg_prefix/bin"
}
