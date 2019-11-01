$ErrorActionPreference="stop" 

Write-Host "--- Fetching manifest"
$baseUrl = "https://packages.chef.io/files/dev/habitat/latest"
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

if($env:BUILDKITE_BUILD_CREATOR -eq $valid_build_creator) {
  choco pack $tempDir/habitat.nuspec --version $Version
  choco push habitat.$Version.nupkg -k $env:CHOCO_API_KEY --timeout 600
} else {
  Write-Host "--- NOT PUBLISHING: Build triggered by $env:BUILDKITE_BUILD_CREATOR and not $valid_build_creator"
  Write-Host "choco pack $tempDir/habitat.nuspec --version $Version"
  Write-Host "choco push habitat.$Version.nupkg -k <elided ChocoApiKey> --timeout 600"
}

Remove-Item $tempDir -Recurse -Force
