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

Write-Host "--- Installing latest core/hab from $env:HAB_BLDR_URL, $Channel channel"
Invoke-NativeCommand hab pkg install core/hab `
    --binlink `
    --force `
    --channel "$Channel" `
    --url="$BuilderUrl"
Write-Host "--- Using core/hab $(hab --version)"

Write-Host "--- Installing latest core/powershell from $env:HAB_BLDR_URL, stable channel"
Invoke-NativeCommand hab pkg install core/powershell `
    --binlink `
    --force `
    --channel=stable `
    --url="$BuilderUrl"
Write-Host "--- Using core/powershell $(pwsh --version)"

Write-Host "--- Installing latest core/pester from $env:HAB_BLDR_URL, stable channel"
Invoke-NativeCommand hab pkg install core/pester `
    --channel=stable `
    --url="$BuilderUrl"
