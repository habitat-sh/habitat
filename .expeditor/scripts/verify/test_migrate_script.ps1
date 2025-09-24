Write-Host "--- Installing pester"
hab pkg install core/pester
Import-Module "$(hab pkg path core/pester)\module\pester.psd1"
$env:HAB_NOCOLORING = "true"

Write-Host "--- Running migrate.ps1 tests"
$configuration = [PesterConfiguration]::Default
$configuration.Run.Exit = $true
$configuration.Run.Path = "components/hab/tests/test_migrate_script.ps1"
Invoke-Pester -Configuration $configuration

