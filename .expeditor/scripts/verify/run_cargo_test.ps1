#!/usr/bin/env powershell

#Requires -Version 5

param (
    # The name of the component to be built. Defaults to none
    [string]$Component,
    # Features to pass to cargo
    [string]$Features,
    # Options to pass to the cargo test command
    [string]$TestOptions,
    [switch]$Nightly
)

$ErrorActionPreference="stop"
. $PSScriptRoot\shared.ps1

$toolchain = Get-Toolchain
if($Nightly) { $toolchain = Get-NightlyToolchain }

Setup-Environment

If($Features) {
    $FeatureString = "--features `"$Features`""
} Else {
    $FeatureString = ""
}

# Set cargo test invocation
Install-Rustup $toolchain
Install-RustToolchain $toolchain
$CargoTestCommand = "cargo +$toolchain test $FeatureString -- $TestOptions"


if(!$Component) {
    $scope = "habitat workspace"
} else {
    $scope = $Component
    cd components/$Component
}
Write-Host "--- Running cargo +$toolchain test on $scope with command: '$CargoTestCommand'"

cargo +$toolchain version
Invoke-Expression $CargoTestCommand

if ($LASTEXITCODE -ne 0) {exit $LASTEXITCODE}
