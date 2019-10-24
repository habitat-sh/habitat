param (
    [string]$Channel="dev",
    [string]$TestName
)

$ErrorActionPreference="stop" 
$env:HAB_LICENSE = "accept-no-persist"
. .expeditor/scripts/shared.ps1

Install-Habitat

Write-Host "--- Installing latest core/hab from $env:HAB_BLDR_URL, $Channel channel"
hab pkg install core/hab `
    --binlink `
    --force `
    --channel "$Channel" `
    --url="$env:HAB_BLDR_URL"
$env:PATH="$env:SystemDrive\hab\bin;$env:PATH"
Write-Host "--- Using $(hab --version)"

hab pkg install core/pester --channel stable
Import-Module "$(hab pkg path core/pester)\module\pester.psd1"
$env:HAB_NOCOLORING = "true"

Invoke-Pester test/end-to-end/${TestName}.ps1 -EnableExit
