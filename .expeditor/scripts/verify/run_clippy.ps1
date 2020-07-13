#Requires -Version 5

param (
    [string]$UnexaminedLintsPath,
    [string]$AllowedLintsPath,
    [string]$LintsToFixPath,
    [string]$DeniedLintsPath
)

$ErrorActionPreference="stop"
. $PSScriptRoot\shared.ps1

$Env:RUSTFLAGS="-D warnings"

function Convert-ArrayToArgList ($arg, $list) {
    if($list) {
        $list | ForEach-Object { "-$arg $_ ``" } | Out-String
    }
}

$toolchain = Get-Toolchain
Install-Rustup $toolchain
Install-RustToolchain $toolchain

Write-Host "Installing clippy"
rustup component add --toolchain "$toolchain" clippy

Initialize-Environment

$clippyArgs += Convert-ArrayToArgList -arg A -list (Get-Content $UnexaminedLintsPath)
$clippyArgs += Convert-ArrayToArgList -arg A -list (Get-Content $AllowedLintsPath)
$clippyArgs += Convert-ArrayToArgList -arg W -list (Get-Content $LintsToFixPath)
$clippyArgs += Convert-ArrayToArgList -arg D -list (Get-Content $DeniedLintsPath)

$clippyCommand = "cargo +$toolchain clippy --all-targets --tests -- $clippyArgs"
Write-Host "--- Running clippy!"
Write-Host "Clippy rules: $clippyCommand"
cargo +$toolchain version
cargo +$toolchain clippy --version
Invoke-Expression $clippyCommand

if ($LASTEXITCODE -ne 0) {exit $LASTEXITCODE}
