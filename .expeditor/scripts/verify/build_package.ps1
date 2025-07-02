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

# Install hab-studio from the chef origin via the staging channel.
# By default, it installs from the stable channel only,
# so this may need updating to support other channels.
$env:HAB_INTERNAL_BLDR_CHANNEL="staging"
hab pkg install chef/hab-studio

Write-Host "--- :hab: Running hab pkg build for $PackagePath"
git config --global --add safe.directory C:/workdir

hab studio build $PackagePath -R

exit $LASTEXITCODE
