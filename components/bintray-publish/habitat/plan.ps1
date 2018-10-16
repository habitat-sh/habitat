$pkg_name = "hab-bintray-publish"
$pkg_origin = "core"
$pkg_version = "$(Get-Content $PLAN_CONTEXT/../../../VERSION)"
$pkg_maintainer = "The Habitat Maintainers <humans@habitat.sh>"
$pkg_license = @("Apache-2.0")
$pkg_source = "nosuchfile.tar.gz"
$pkg_deps=@("core/powershell", "core/docker", "core/hab", "core/docker-credential-helper")
$pkg_bin_dirs = @("bin")

function Invoke-Build {
    Get-Content "$PLAN_CONTEXT/../bin/publish-studio.ps1" | % {
        $_.Replace("@author@", $pkg_maintainer).Replace("@version@", "$pkg_version/$pkg_release")
      } | Add-Content -Path publish-studio.ps1
}

function Invoke-Install {
    Copy-Item "$PLAN_CONTEXT/../../studio/build-docker-image.ps1" "$pkg_prefix/bin"
    Copy-Item publish-studio.ps1 "$pkg_prefix/bin"
    Copy-Item $PLAN_CONTEXT/../bin/publish-studio.bat "$pkg_prefix/bin"
}

function Invoke-Download {}
function Invoke-Verify {}
function Invoke-Unpack {}
