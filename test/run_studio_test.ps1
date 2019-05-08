#Requires -Version 5

param (
    [string]$studio_type
)

$ErrorActionPreference="stop"

if(!(Get-Module Await -ListAvailable)) {
  Write-Host "Installing Await PS Module..."
  Install-Module Await -Force | Out-Null
}

Push-Location "components/studio"
try {
 & test/$studio_type/test.ps1
}
finally { Pop-Location }

if ($LASTEXITCODE -ne 0) {exit $LASTEXITCODE}

