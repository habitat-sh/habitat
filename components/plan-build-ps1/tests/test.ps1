. $PSScriptRoot\..\..\..\.expeditor\scripts\verify\shared.ps1
$env:HAB_LICENSE = "accept-no-persist"
Install-Habitat

hab pkg install core/pester
Import-Module "$(hab pkg path core/pester)\module\pester.psd1"

Invoke-Pester $PSScriptRoot -EnableExit
