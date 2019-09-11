param (
    [string]$Channel="DEV"
)

$ErrorActionPreference="stop" 

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


