. $PSScriptRoot\shared.ps1
Install-Habitat

Write-Host "--- Installing pester"
Install-HabPkg -idents @("core/pester")
Import-Module "$(hab pkg path core/pester)\module\pester.psd1"
$env:HAB_NOCOLORING = "true"

Write-Host "--- Running install.ps1 tests"
Invoke-Pester components/hab/tests/test_install_script.ps1 -EnableExit
