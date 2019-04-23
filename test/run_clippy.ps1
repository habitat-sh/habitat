#Requires -Version 5

param (
    [string]$toolchain = "stable",
    [string]$UnexaminedLintsPath,
    [string]$AllowedLintsPath,
    [string]$LintsToFixPath,
    [string]$DeniedLintsPath
)

$ErrorActionPreference="stop"
. $PSScriptRoot\..\support\ci\shared.ps1

function Convert-ArrayToArgs ($arg, $list) {
    if($list) {
        $list | ForEach-Object { "-$arg $_ ``" } | Out-String
    }
}

Install-Rustup $toolchain
Install-RustToolchain $toolchain

Write-Host "Installing clippy"
rustup component add clippy

Setup-Environment

$clippyArgs += Convert-ArrayToArgs -arg A -list (Get-Content $UnexaminedLintsPath)
$clippyArgs += Convert-ArrayToArgs -arg A -list (Get-Content $AllowedLintsPath)
$clippyArgs += Convert-ArrayToArgs -arg W -list (Get-Content $LintsToFixPath)
$clippyArgs += Convert-ArrayToArgs -arg D -list (Get-Content $DeniedLintsPath)

$clippyCommand = "cargo +$toolchain clippy --all-targets --tests -- $clippyArgs"
Write-Host "--- Running clippy!"
Write-Host "Clippy rules: $clippyCommand"
Invoke-Expression $clippyCommand

if ($LASTEXITCODE -ne 0) {exit $LASTEXITCODE}
