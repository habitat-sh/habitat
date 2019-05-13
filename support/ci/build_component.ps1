#!/usr/bin/env powershell

#Requires -Version 5

param (
    # The name of the component to be built. Defaults to none
    [string]$Component
)

# Since we are only verifying we don't have build failures, make everything
# temp!
$env:HAB_ORIGIN="throwaway"
# let's make a selfcontained tempdir for this job
$job_temp_root = mkdir (Join-Path $env:TEMP ([System.IO.Path]::GetRandomFileName()))
$env:HAB_CACHE_KEY_PATH="$job_temp_root/keys"

Write-Host "--- :key: Generating fake origin key"
hab origin key generate
Write-Host "--- :hab: Running hab pkg build for $Component"

hab studio build -D --no-tty --non-interactive components/$Component

exit $LASTEXITCODE
