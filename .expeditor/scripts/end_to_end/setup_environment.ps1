param (
    [string]$Channel = "dev",
    [string]$BuilderUrl = $env:HAB_BLDR_URL
)

$ErrorActionPreference = "stop"

. .expeditor/scripts/shared.ps1
$env:HAB_NOCOLORING = "true"
$env:HAB_LICENSE = "accept-no-persist"

Install-Habitat
$binPath = Join-Path -Path $env:SystemDrive -ChildPath hab | Join-Path -ChildPath bin
$env:PATH = New-PathString $env:PATH $binPath

Write-Host "--- Installing latest chef/hab from $BuilderUrl, $Channel channel"
Invoke-NativeCommand hab pkg install chef/hab `
    --force `
    --channel "$Channel" `
    --url="$BuilderUrl"
$habPath = hab pkg path chef/hab
Invoke-NativeCommand "$habPath/bin/hab.exe" pkg binlink chef/hab --force
Write-Host "--- Using chef/hab $(hab --version)"

Write-Host "--- Installing latest chef/hab-pkg-export-container from $BuilderUrl, $Channel channel"
Invoke-NativeCommand hab pkg install chef/hab-pkg-export-container `
    --channel "$Channel" `
    --url="$BuilderUrl"

Write-Host "--- Installing latest core/powershell from $BuilderUrl, stable channel"
Invoke-NativeCommand hab pkg install core/powershell `
    --binlink `
    --force `
    --channel=unstable `
    --url="$BuilderUrl"
Write-Host "--- Using core/powershell $(pwsh --version)"

Write-Host "--- Installing latest core/pester from $BuilderUrl, stable channel"
# use --ignore-local because there is a local base-2025 pester package that is broken
Invoke-NativeCommand hab pkg install core/pester `
    --channel=stable `
    --ignore-local `
    --url="$BuilderUrl"
