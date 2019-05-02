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
. $PSScriptRoot\..\support\ci\shared.ps1

$toolchain = "stable"
if($Nightly) { $toolchain = "nightly" }

Install-Rustup $toolchain
Install-RustToolchain $toolchain

If($Features) {
    $FeatureString = "--features `"$Features`""
} Else {
    $FeatureString = ""
}

# Set cargo test invocation
$CargoTestCommand = "cargo +$toolchain test --quiet $FeatureString -- $TestOptions"

Setup-Environment

Write-Host "--- Running cargo test on $Component with command: '$CargoTestCommand'"
cd components/$Component
Invoke-Expression $CargoTestCommand

if ($LASTEXITCODE -ne 0) {exit $LASTEXITCODE}
