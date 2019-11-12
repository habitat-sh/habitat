#Requires -Version 5

param (
    [string]$studio_type
)

$ErrorActionPreference="stop"

. $PSScriptRoot\shared.ps1
Install-Habitat

if(!(Get-PackageProvider -Name nuget -ErrorAction SilentlyContinue -ListAvailable)) {
    Write-Host "Installing Nuget provider..."
    Install-PackageProvider -Name NuGet -Force | Out-Null
}

if(!(Get-Module Await -ListAvailable)) {
  Write-Host "Installing Await PS Module..."
  Install-Module Await -Force | Out-Null
}

# Studio test uses relative paths to copy necessary files and expects to be run
# from the studio source directory
Push-Location "components/studio"
try {
 & test/test.ps1
}
finally { Pop-Location }

if ($LASTEXITCODE -ne 0) {exit $LASTEXITCODE}

