$ErrorActionPreference="stop"

Write-Host "--- Fetching manifest"
$baseUrl = "https://packages.chef.io/files/stable/habitat/latest"
$manifestUrl = "$baseUrl/manifest.json"
Write-Host "Fetching $manifestUrl"
$manifest = (Invoke-WebRequest -URI "$manifestUrl" -ErrorAction Stop -UseBasicParsing).Content | ConvertFrom-Json

Write-Host "--- :thinking_face: Determining version and release to publish"
$Version = ($manifest.version).Trim()
Write-Host $Version

Write-Host "--- Fetching checksum"
$shasumUrl= "$baseUrl/hab-x86_64-windows.zip.sha256sum"
Write-Host $shasumUrl
$Checksum = (Invoke-RestMethod -Uri "$shasumUrl" -ErrorAction Stop -UseBasicParsing).Split(' ')[0]
Write-Host $Checksum

Write-Host "--- Patching nuspec"
$tempDir = Join-Path $env:temp ([System.IO.Path]::GetRandomFileName())
New-Item $tempDir -ItemType Directory -Force | Out-Null
Copy-Item "components/hab/win/*" $tempDir
$choco_install = Join-Path $tempDir chocolateyinstall.ps1

(Get-Content $choco_install) |
    % {$_.Replace('$version$', $Version) } |
    Set-Content $choco_install

(Get-Content $choco_install) |
    % {$_.Replace('$checksum$', $Checksum) } |
    Set-Content $choco_install

Write-Host "--- :bug: DEBUG PRINT PATCHED NUSPEC"
Get-Content $choco_install | Write-Host

Write-Host "--- Publishing package"

$valid_build_creator="Chef Expeditor"

$pack_cmd = "choco pack $tempDir/habitat.nuspec --version $Version"
$publish_cmd = "choco push habitat.$Version.nupkg --timeout 600"

try {
    if($env:BUILDKITE_BUILD_CREATOR -eq $valid_build_creator) {
        Invoke-Expression $pack_cmd
        if (-not $?) { throw "unable to choco pack" }
        Invoke-Expression $publish_cmd " --key " $env:CHOCO_API_KEY
        if (-not $?) { throw "unable to publish Chocolatey package" }
    } else {
        Write-Host "--- NOT PUBLISHING: Build triggered by $env:BUILDKITE_BUILD_CREATOR and not $valid_build_creator"
        Write-Host $pack_cmd
        Write-Host $publish_cmd " --key <elided ChocoApiKey>"
    }
} finally {
    Remove-Item $tempDir -Recurse -Force
}
