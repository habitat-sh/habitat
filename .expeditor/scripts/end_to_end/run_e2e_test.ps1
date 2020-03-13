param (
    [string]$Channel = "dev",
    [string]$TestName,
    [string]$BuilderUrl = $env:HAB_BLDR_URL
)

$count = 0
while($count -lt 30) {
    $count +=1
    Write-Host "LAST EXIT: $LASTEXITCODE"
    Start-Sleep -Seconds 10
}
