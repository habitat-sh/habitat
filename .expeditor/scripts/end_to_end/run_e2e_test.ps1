param (
    [string]$Channel = "dev",
    [string]$TestName,
    [string]$BuilderUrl = $env:HAB_BLDR_URL
)
write-host "starting..."
. .expeditor/scripts/shared.ps1
# $count = 0
# while($count -lt 30) {
#     $count +=1
#     Write-Host "LAST EXIT: $LASTEXITCODE"
#     Start-Sleep -Seconds 10
# }

. .expeditor/scripts/end_to_end/setup_environment.ps1 $Channel $BuilderUrl
Invoke-NativeCommand pwsh .expeditor/scripts/end_to_end/run_e2e_test_core.ps1 $TestName $BuilderUrl
