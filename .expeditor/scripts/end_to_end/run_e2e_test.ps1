param (
    [string]$Channel = "dev",
    [string]$TestName,
    [string]$BuilderUrl = $env:HAB_BLDR_URL
)

Write-Host "LAST EXIT: $LASTEXITCODE"
. .expeditor/scripts/shared.ps1
. .expeditor/scripts/end_to_end/setup_environment.ps1 $Channel $BuilderUrl
Write-Host "LAST EXIT: $LASTEXITCODE"
Invoke-NativeCommand pwsh .expeditor/scripts/end_to_end/run_e2e_test_core.ps1 $TestName $BuilderUrl
