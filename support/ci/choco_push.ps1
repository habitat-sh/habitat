param (
    [string]$Version,
    [string]$Release,
    [string]$Checksum,
    [string]$ApiKey
)

function Get-RepoRoot {
    (Resolve-Path "$PSScriptRoot\..\..\").Path
}

$versionStamp = "$Version-$Release"
$tempDir = Join-Path $env:temp ([System.IO.Path]::GetRandomFileName())
New-Item $tempDir -ItemType Directory -Force | Out-Null
Copy-Item "$(Get-RepoRoot)/components/hab/win/*" $tempDir
$choco_install = Join-Path $tempDir chocolateyinstall.ps1

(Get-Content $choco_install) |
    % {$_.Replace('$version$', $versionStamp) } |
    Set-Content $choco_install

(Get-Content $choco_install) |
    % {$_.Replace('$checksum$', $Checksum) } |
    Set-Content $choco_install

choco pack "$tempDir/habitat.nuspec" --version $Version
choco push habitat.$Version.nupkg -k $ApiKey --timeout 600
Remove-Item $tempDir -Recurse -Force
