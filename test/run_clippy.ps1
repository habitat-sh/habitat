#Requires -Version 5

param (
    [string]$toolchain = "stable",
    [string]$UnexaminedLintsPath,
    [string]$AllowedLintsPath,
    [string]$LintsToFixPath,
    [string]$DeniedLintsPath
)

$ErrorActionPreference="stop"

function Convert-ArrayToArgs ($arg, $list) {
    if($list) {
        $list | ForEach-Object { "-$arg $_ ``" } | Out-String
    }
}

# On buildkite, the rust binaries will be directly in C:
if(Test-Path C:\rust\.cargo) {
    $env:CARGO_HOME="C:\rust\.cargo"
    $env:path += ";C:\rust\.cargo\bin"
    # this will avoid a path length limit from the long buildkite working dir path
    $env:CARGO_TARGET_DIR = "c:\target"
}
if(Test-Path C:\rust\.rustup) {
    $env:RUSTUP_HOME="C:\rust\.rustup"
    $env:path += ";C:\rust\.rustup\bin"
}

# For normal developers
if(Test-Path $env:USERPROFILE\.cargo\bin) {
    $env:path += ";$env:USERPROFILE\.cargo\bin"
}
if (!(get-command -Name rustup.exe -ErrorAction SilentlyContinue)) {
    Write-Host "Installing rustup and $toolchain-x86_64-pc-windows-msvc Rust."
    [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
    invoke-restmethod -usebasicparsing 'https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe' -outfile 'rustup-init.exe'
    ./rustup-init.exe -y --default-toolchain $toolchain-x86_64-pc-windows-msvc --no-modify-path
    $env:path += ";$env:USERPROFILE\.cargo\bin"
}

rustup component list --toolchain $toolchain | Out-Null
if ($LASTEXITCODE -ne 0) {
    Write-Host "Installing rust toolchain $toolchain"
    rustup toolchain install $toolchain
}

Write-Host "Installing clippy"
rustup component add clippy

# TODO: these should be in a shared script?
Write-Host "--- Installing required prerequisites"
hab pkg install core/cacerts
hab pkg install core/libarchive
hab pkg install core/libsodium
hab pkg install core/openssl
hab pkg install core/protobuf
hab pkg install core/xz
hab pkg install core/zeromq
hab pkg install core/zlib
hab pkg install core/visual-cpp-build-tools-2015

# Set up some path variables for ease of use later
$cacertsDir     = & hab pkg path core/cacerts
$libarchiveDir  = & hab pkg path core/libarchive
$libsodiumDir   = & hab pkg path core/libsodium
$opensslDir     = & hab pkg path core/openssl
$protobufDir    = & hab pkg path core/protobuf
$xzDir          = & hab pkg path core/xz
$zeromqDir      = & hab pkg path core/zeromq
$zlibDir        = & hab pkg path core/zlib
$vsDir          = & hab pkg path core/visual-cpp-build-tools-2015

# Set some required variables
$env:SODIUM_LIB_DIR             = "$libsodiumDir\lib"
$env:LIBARCHIVE_INCLUDE_DIR     = "$libarchiveDir\include"
$env:LIBARCHIVE_LIB_DIR         = "$libarchiveDir\lib"
$env:OPENSSL_LIBS               = 'ssleay32:libeay32'
$env:OPENSSL_LIB_DIR            = "$opensslDir\lib"
$env:OPENSSL_INCLUDE_DIR        = "$opensslDir\include"
$env:LIBZMQ_PREFIX              = "$zeromqDir"
$env:SSL_CERT_FILE              = "$cacertsDir\ssl\certs\cacert.pem"
$env:SODIUM_STATIC              = "true"
$env:OPENSSL_STATIC             = "true"
$env:LD_LIBRARY_PATH            = "$env:LIBZMQ_PREFIX\lib;$env:SODIUM_LIB_DIR;$zlibDir\lib;$xzDir\lib"
$env:LIB                        = (Get-Content "$vsDir\LIB_DIRS")
$env:INCLUDE                    = (Get-Content "$vsDir\INCLUDE_DIRS")
$env:PATH                       += ";$(Get-Content "$vsDir\PATH")"

# Make sure protoc is on the path, we also need to make sure the DLLs (in \bin) are on the path,
# because windows library pathing is weird and terrifying.
$env:Path="$env:Path;$protobufDir\bin;$zeromqDir\bin;$libarchiveDir\bin;$libsodiumDir\bin;$zlibDir\bin;$xzDir\bin;$opensslDir\bin"

$clippyArgs += Convert-ArrayToArgs -arg A -list (Get-Content $UnexaminedLintsPath)
$clippyArgs += Convert-ArrayToArgs -arg A -list (Get-Content $AllowedLintsPath)
$clippyArgs += Convert-ArrayToArgs -arg W -list (Get-Content $LintsToFixPath)
$clippyArgs += Convert-ArrayToArgs -arg D -list (Get-Content $DeniedLintsPath)

$clippyCommand = "cargo +$toolchain clippy --all-targets --tests -- $clippyArgs"
Write-Host "--- Running clippy!"
Write-Host "Clippy rules: $clippyCommand"
Invoke-Expression $clippyCommand

if ($LASTEXITCODE -ne 0) {exit $LASTEXITCODE}
