#!/usr/bin/env powershell

#Requires -Version 5

## Helper Functions
function New-PathString([string]$StartingPath, [string]$Path) {
    if (-not [string]::IsNullOrEmpty($path)) {
        if (-not [string]::IsNullOrEmpty($StartingPath)) {
            [string[]]$PathCollection = "$path;$StartingPath" -split ';'
            $Path = ($PathCollection |
              Select-Object -Unique | 
              where {-not [string]::IsNullOrEmpty($_.trim())} | 
              where {test-path "$_"}
              ) -join ';'
        }
      $path
    }
}

function Test-AppVeyor {
    (test-path env:\APPVEYOR) -and ([bool]::Parse($env:APPVEYOR))
} 

# Make sure that chocolatey is installed and up to date
# (required for dependencies)
if (-not (get-command choco -ErrorAction SilentlyContinue)) {
    Write-Host "Installing Chocolatey"
    Invoke-Expression ((New-Object System.Net.WebClient).DownloadString('https://chocolatey.org/install.ps1')) | out-null
}
else {
    Write-Host "Making sure Chocolatey is current."
    choco upgrade chocolatey --confirm | Out-Null
}

# We need the native library dependencies for `hab`
# Until we have habitat packages on Windows, there is
# a chocolatey package hosted in MyGet with the native 
# dependencies built.
choco install habitat_native_dependencies --confirm -s https://www.myget.org/F/habitat/api/v2  --allowemptychecksums

# set a few reference variables for later
$ChocolateyHabitatLibDir = "$env:ChocolateyInstall\lib\habitat_native_dependencies\builds\lib"
$ChocolateyHabitatIncludeDir = "$env:ChocolateyInstall\lib\habitat_native_dependencies\builds\include"
$ChocolateyHabitatBinDir = "$env:ChocolateyInstall\lib\habitat_native_dependencies\builds\bin"

if (-not (Test-AppVeyor)) {
    # We need the Visual C 2013 Runtime for the Win32 ABI Rust
    choco install 'vcredist2013' --confirm --allowemptychecksum

    # We need the Visual C++ tools to build Rust crates (provides a compiler and linker) 
    choco install 'visualcppbuildtools' --version '14.0.25123' --confirm --allowemptychecksum
}

# Install Rust Nightly (since there aren't MSVC nightly cargo builds)
if (get-command -Name rustup.exe -ErrorAction SilentlyContinue) {
    rustup install nightly-x86_64-pc-windows-msvc
    $cargo = 'rustup run nightly-x86_64-pc-windows-msvc cargo'
}
else {
    $env:PATH = New-PathString -StartingPath $env:PATH -Path "C:\Program Files (x86)\Rust\bin"
    if (-not (get-command rustc -ErrorAction SilentlyContinue)) {
        write-host "installing rust"
        Invoke-WebRequest -UseBasicParsing -Uri 'https://static.rust-lang.org/dist/rust-nightly-x86_64-pc-windows-msvc.exe' -OutFile './rust-nightly.exe'
        start-process -filepath ./rust-nightly.exe -argumentlist  '/VERYSILENT', '/NORESTART', '/DIR="C:\Program Files (x86)\Rust"' -Wait
        $env:PATH = New-PathString -StartingPath $env:PATH -Path "C:\Program Files (x86)\Rust\bin"
        while (-not (get-command cargo -ErrorAction SilentlyContinue)) {
            Write-Warning "`tWaiting for `cargo` to be available."
            start-sleep -Seconds 1
        }
        
    }
    else {
        # TODO: version checking logic and upgrades
    }
    $cargo = 'cargo'
}

# Set Environment Variables for the build
$env:LIB                        = New-PathString -StartingPath $env:LIB     -Path $ChocolateyHabitatLibDir
$env:INCLUDE                    = New-PathString -StartingPath $env:INCLUDE -Path $ChocolateyHabitatIncludeDir
$env:PATH                       = New-PathString -StartingPath $env:PATH    -Path $ChocolateyHabitatBinDir
$env:SODIUM_STATIC              = $true
$env:SODIUM_LIB_DIR             = $ChocolateyHabitatLibDir
$env:LIBARCHIVE_INCLUDE_DIR     = $ChocolateyHabitatIncludeDir
$env:LIBARCHIVE_LIB_DIR         = $ChocolateyHabitatLibDir
$env:OPENSSL_LIBS               = 'ssleay32:libeay32'
$env:OPENSSL_LIB_DIR            = $ChocolateyHabitatLibDir
$env:OPENSSL_INCLUDE_DIR        = $ChocolateyHabitatIncludeDir
$env:OPENSSL_STATIC             = $true


# Start the build
if (-not (Test-AppVeyor)) {
    Push-Location "$psscriptroot\.."
    invoke-expression "$cargo clean"
    Invoke-Expression "$cargo build" 
    Pop-Location
    exit $LASTEXITCODE
}