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
if($Nightly) { $toolchain = (gc $PSScriptRoot\..\RUSTFMT_VERSION | out-string).Trim() }

Setup-Environment

If($Features) {
    $FeatureString = "--features `"$Features`""
} Else {
    $FeatureString = ""
}

# Set cargo test invocation
if($Nightly) {
    Install-Rustup $toolchain
    Install-RustToolchain $toolchain
    $CargoTestCommand = "cargo +$toolchain"
} else {
    $env:path = "$(hab pkg path core/rust)\bin;$env:path"
    $CargoTestCommand = "cargo"
}
$CargoTestCommand += " test $FeatureString -- $TestOptions"

Write-Host "--- Running cargo test on $Component with command: '$CargoTestCommand'"
cd components/$Component
Invoke-Expression $CargoTestCommand

if ($LASTEXITCODE -ne 0) {exit $LASTEXITCODE}
