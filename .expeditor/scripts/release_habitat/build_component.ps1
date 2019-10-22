#!/usr/bin/env powershell

#Requires -Version 5

param (
    # The name of the component to be built. Defaults to none
    [string]$Component
)

$ErrorActionPreference="stop"

# Import shared functions
. $PSScriptRoot\shared.ps1

if($Component.Equals("")) {
    Write-Error "--- :error: Component to build not specified, please use the -Component flag"
}

# We have to do this because everything that comes from vault is quoted on windows.
# TODO: This can be removed when we go live!
$Rawtoken=$Env:ACCEPTANCE_HAB_AUTH_TOKEN
$Env:HAB_AUTH_TOKEN=$Rawtoken.Replace("`"","")

$Env:buildkiteAgentToken = $Env:BUILDKITE_AGENT_ACCESS_TOKEN

$Env:HAB_BLDR_URL=$Env:ACCEPTANCE_HAB_BLDR_URL
$Env:HAB_PACKAGE_TARGET=$Env:BUILD_PKG_TARGET

Install-BuildkiteAgent

# Install jq if it doesn't exist
choco install jq -y | Out-Null

# For viewability
$Channel = "habitat-release-$Env:BUILDKITE_BUILD_ID"
Write-Host "--- Channel: $Channel - bldr url: $Env:HAB_BLDR_URL"

$baseHabExe=Install-LatestHabitat

# Get keys
Write-Host "--- :key: Downloading 'core' public keys from Builder"
Invoke-Expression "$baseHabExe origin key download core"
Write-Host "--- :closed_lock_with_key: Downloading latest 'core' secret key from Builder"
Invoke-Expression "$baseHabExe origin key download core --auth $Env:HAB_AUTH_TOKEN --secret"
$Env:HAB_CACHE_KEY_PATH = "C:\hab\cache\keys"
$Env:HAB_ORIGIN = "core"

# This is a temporary measure so we can run fake releases
$Env:HAB_STUDIO_SECRET_DO_FAKE_RELEASE=$Env:DO_FAKE_RELEASE

# Run a build!
Write-Host "--- Running hab pkg build for $Component"

# Note: HAB_BLDR_CHANNEL *must* be set for the following `hab pkg
# build` command! There isn't currently a CLI option to set that, and
# we must ensure that we're pulling dependencies from our build
# channel when applicable.
$Env:HAB_BLDR_CHANNEL="$Channel"
Invoke-Expression "$baseHabExe pkg build components\$Component --keys core"
. results\last_build.ps1

Write-Host "--- Running hab pkg upload for $Component to channel $Channel"
Invoke-Expression "$baseHabExe pkg upload results\$pkg_artifact --channel=$Channel"
Set-TargetMetadata $pkg_ident

Invoke-Expression "buildkite-agent annotate --append --context 'release-manifest' '<br>* ${pkg_ident} (x86_64-windows)'"

exit $LASTEXITCODE
