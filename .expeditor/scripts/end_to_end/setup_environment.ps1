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

Write-Host "--- Installing latest core/hab from $BuilderUrl, $Channel channel"
Invoke-NativeCommand hab pkg install core/hab `
    --binlink `
    --force `
    --channel "$Channel" `
    --url="$BuilderUrl"
Write-Host "--- Using core/hab $(hab --version)"

Write-Host "--- Installing latest core/hab-pkg-export-container from $BuilderUrl, $Channel channel"
Invoke-NativeCommand hab pkg install core/hab-pkg-export-container `
    --channel "$Channel" `
    --url="$BuilderUrl"

Write-Host "--- Installing latest core/powershell from $BuilderUrl, LTS-2024 channel"
Invoke-NativeCommand hab pkg install core/powershell `
    --binlink `
    --force `
    --channel=LTS-2024 `
    --url="$BuilderUrl"
Write-Host "--- Using core/powershell $(pwsh --version)"

Write-Host "--- Installing latest core/pester from $BuilderUrl, LTS-2024 channel"
Invoke-NativeCommand hab pkg install core/pester `
    --channel=LTS-2024 `
    --url="$BuilderUrl"
