#!/usr/bin/env powershell

#Requires -Version 5

param (
    # The path to the package to be built. Defaults to none
    [string]$PackagePath
)

. $PSScriptRoot\shared.ps1
Install-Habitat

# Since we are only verifying we don't have build failures, make everything
# temp!
$env:HAB_ORIGIN="throwaway"
# let's make a selfcontained tempdir for this job
$job_temp_root = mkdir (Join-Path $env:TEMP ([System.IO.Path]::GetRandomFileName()))
$env:HAB_CACHE_KEY_PATH="$job_temp_root/keys"

Write-Host "--- :key: Generating fake origin key"
hab origin key generate
Write-Host "--- :hab: Running hab pkg build for $PackagePath"

hab studio build $PackagePath -R

exit $LASTEXITCODE
