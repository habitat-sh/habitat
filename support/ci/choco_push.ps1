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
$choco_install = "$(Get-RepoRoot)/components/hab/win/chocolateyinstall.ps1"

(Get-Content $choco_install) |
    % {$_.Replace('$version$', $versionStamp) } |
    Set-Content $choco_install

(Get-Content $choco_install) |
    % {$_.Replace('$checksum$', $Checksum) } |
    Set-Content $choco_install

choco pack "$(Get-RepoRoot)/components/hab/win/habitat.nuspec" --version $Version 
choco push habitat.$Version.nupkg -k $ApiKey --timeout 600
