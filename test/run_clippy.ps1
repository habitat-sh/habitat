#Requires -Version 5

param (
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

$toolchain = Rust-Toolchain
Install-Rustup $toolchain
Install-RustToolchain $toolchain

Write-Host "Installing clippy"
rustup component add --toolchain "$toolchain" clippy

Setup-Environment

$clippyArgs += Convert-ArrayToArgs -arg A -list (Get-Content $UnexaminedLintsPath)
$clippyArgs += Convert-ArrayToArgs -arg A -list (Get-Content $AllowedLintsPath)
$clippyArgs += Convert-ArrayToArgs -arg W -list (Get-Content $LintsToFixPath)
$clippyArgs += Convert-ArrayToArgs -arg D -list (Get-Content $DeniedLintsPath)

$clippyCommand = "cargo +$toolchain clippy --all-targets --tests -- $clippyArgs"
Write-Host "--- Running clippy!"
Write-Host "Clippy rules: $clippyCommand"
cargo +$toolchain version
cargo +$toolchain clippy --version
Invoke-Expression $clippyCommand

if ($LASTEXITCODE -ne 0) {exit $LASTEXITCODE}
