#!/usr/bin/env powershell

#Requires -Version 5

<#
.SYNOPSIS
Builds Habitat components for Windows

.DESCRIPTION
This script builds habitat components and ensures that all necesary prerequisites are installed.
#>

param (
    # The path to the component to be built. If not specified the current directory is used.
    [string]$Path=".",
    # When specified, all necessary prerequisites will be installed.
    [switch]$Configure,
    # When specified, a cargo clean will be invoked.
    [switch]$Clean,
    # Cargo command to invoke
    [ValidateSet("Build","Test","Check","Clippy","Fmt")] 
    [string]$command="Build",
    # When specified a build will not be run.
    [switch]$SkipBuild,
    # Use a optimized release build
    [switch]$Release,
    # Features to pass to cargo
    [string]$Features,
    # Options to pass to the cargo test command
    [string]$TestOptions,
    # The Rust toolchain to use and enjoy
    [string]$Toolchain="stable"
)
$ErrorActionPreference="stop"
. $PSScriptRoot\support\ci\shared.ps1

if($Command -eq "Fmt") {
    $toolchain = "$(Get-Content $PSScriptRoot/RUSTFMT_VERSION)"
    Write-Host "Forcing the use of $toolchain toolchain for rustfmt"
}

if(!$env:ChocolateyInstall) {
    $env:ChocolateyInstall = "$env:ProgramData\Chocolatey"
}

function Invoke-Configure {
    # Make sure that chocolatey is installed and up to date
    # (required for dependencies)
    if (-not (get-command choco -ErrorAction SilentlyContinue)) {
        Write-Host "Installing Chocolatey"
        Invoke-Expression ((New-Object System.Net.WebClient).DownloadString('https://chocolatey.org/install.ps1')) | out-null
    }

    if (!((choco list habitat --local-only) -match '^1 packages installed\.$')) {
        choco install habitat -y
    }

    if(!(Get-Command git -ErrorAction SilentlyContinue)) {
        choco install git --confirm
        $env:path = New-PathString -StartingPath $env:path -Path "c:\Program Files\git\cmd"
    }
}

function Get-Component($path) {
    $leaf = Split-Path $path -leaf
    $parent = Split-Path (Split-Path $path -Parent) -leaf
    if($parent -eq "components") { $leaf } else { Write-Error "The specified path is not a component" }
}

function Invoke-Build([string]$Path, [switch]$Clean, [string]$Command, [switch]$Release, [string]$ToolChain, [string]$Features, [string]$TestOptions) {
    $Path = Resolve-Path $Path
    $Command = $command.ToLower()
    if($Features) {
        $FeatureString = "--features $Features"
    } else {
        $FeatureString = ""
    }

    Push-Location "$Path"
    if($Clean) {
        cargo clean
    }

    switch($Command) {
        "fmt" {
            Install-Rustup $toolchain
            Install-RustToolchain $toolchain
            rustup component add --toolchain $Toolchain rustfmt
            Setup-Environment
            Invoke-Expression "cargo +$ToolChain $Command --all"
            break
        }
        "clippy" {
            & $PSScriptRoot\test\run_clippy.ps1 -ToolChain $toolchain `
              -UnexaminedLintsPath $PSScriptRoot\test\unexamined_lints.txt `
              -AllowedLintsPath $PSScriptRoot\test\allowed_lints.txt `
              -LintsToFixPath $PSScriptRoot\test\lints_to_fix.txt `
              -DeniedLintsPath $PSScriptRoot\test\denied_lints.txt `
            break
        }
        "test" {
            $nightly = ($toolchain -eq "nightly")
            Push-Location $PSScriptRoot
            try {
                & $PSScriptRoot\test\run_cargo_test.ps1 -Component (Get-Component $path) -Features $Features -TestOptions $TestOptions -Nightly:$nightly
            } finally { Pop-Location }
            break
        }
        "build" {
            Install-Rustup $toolchain
            Install-RustToolchain $toolchain
            Setup-Environment
            Invoke-Expression "cargo +$ToolChain $Command $(if ($Release) { '--release' }) $FeatureString"
        }
        "check" {
            Install-Rustup $toolchain
            Install-RustToolchain $toolchain
            Setup-Environment
            Invoke-Expression "cargo +$ToolChain $Command $(if ($Release) { '--release' }) $FeatureString"
        }
    }
    Pop-Location
}

if($Configure) {
    Invoke-Configure
}

if (!$SkipBuild) {
    Invoke-Build $Path -Clean:$Clean -Release:$Release -Command $Command -ToolChain $toolchain -Features $Features -TestOptions $TestOptions
}

exit $LASTEXITCODE
