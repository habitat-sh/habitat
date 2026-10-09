#!/usr/bin/env powershell

#Requires -Version 5

param (
    # The name of the component to be built. Defaults to none
    [string]$Component,
    [string]$ReleaseChannel,
    [switch]$SkipBuildkiteReporting
)

$ErrorActionPreference="stop"

# Import shared functions
. $PSScriptRoot\shared.ps1

if($Component.Equals("")) {
    Write-Error "--- :error: Component to build not specified, please use the -Component flag"
}

if ([string]::IsNullOrWhiteSpace($Env:BUILD_PKG_TARGET)) {
    throw "BUILD_PKG_TARGET must be set"
}
if ([string]::IsNullOrWhiteSpace($Env:HAB_AUTH_TOKEN)) {
    throw "HAB_AUTH_TOKEN must be set to publish release packages"
}

# We have to do this because everything that comes from vault is quoted on windows.
$Rawtoken=$Env:HAB_AUTH_TOKEN
$Env:HAB_AUTH_TOKEN=$Rawtoken.Replace("`"","")

$Env:HAB_BLDR_URL=$Env:PIPELINE_HAB_BLDR_URL
$Env:HAB_PACKAGE_TARGET=$Env:BUILD_PKG_TARGET

$Channel = if ([string]::IsNullOrWhiteSpace($ReleaseChannel)) {
    Get-ReleaseChannel
} else {
    $ReleaseChannel
}

if (-not $SkipBuildkiteReporting) {
    $Env:buildkiteAgentToken = $Env:BUILDKITE_AGENT_ACCESS_TOKEN
    Install-BuildkiteAgent
}

Write-Host "--- Channel: $Channel - bldr url: $Env:HAB_BLDR_URL"

# Note: HAB_BLDR_CHANNEL *must* be set for the following `hab pkg
# build` command! There isn't currently a CLI option to set that, and
# we must ensure that we're pulling dependencies from our build
# channel when applicable.
$Env:HAB_BLDR_CHANNEL="$Channel"

$baseHabExe=Install-LatestHabitat

# Get keys
Write-Host "--- :key: Downloading 'chef' public keys from Builder"
Invoke-Expression "$baseHabExe origin key download chef"
Write-Host "--- :closed_lock_with_key: Downloading latest 'chef' secret key from Builder"
Invoke-Expression "$baseHabExe origin key download chef --auth $Env:HAB_AUTH_TOKEN --secret"
$Env:HAB_CACHE_KEY_PATH = "C:\hab\cache\keys"
$Env:HAB_ORIGIN = "chef"

# Run a build!
Write-Host "--- Running hab pkg build for $Component"
if (-not $SkipBuildkiteReporting) {
    git config --global --add safe.directory C:/workdir
}

Invoke-Expression "$baseHabExe pkg build components\$Component --keys chef"
. results\last_build.ps1

Write-Host "--- Running hab pkg upload for $Component to channel $Channel"
Invoke-Expression "$baseHabExe pkg upload results\$pkg_artifact --channel=$Channel"
if ($LASTEXITCODE -ne 0) {exit $LASTEXITCODE}

$result = [PSCustomObject]@{
    component = $Component
    ident = $pkg_ident
    target = $Env:BUILD_PKG_TARGET
    channel = $Channel
}
$resultPath = Join-Path (Get-Location) "results\release-build-result.json"
if ($SkipBuildkiteReporting) {
    $result | ConvertTo-Json -Compress | Set-Content -Path $resultPath -Encoding ascii
} else {
    Set-TargetMetadata $pkg_ident $Env:BUILD_PKG_TARGET
    Invoke-Expression "buildkite-agent annotate --append --context 'release-manifest' '<br>* ${pkg_ident} ($Env:BUILD_PKG_TARGET)'"
}

exit $LASTEXITCODE
