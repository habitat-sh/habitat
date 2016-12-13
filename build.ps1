#!/usr/bin/env powershell

#Requires -Version 5

<#
.SYNOPSIS
Builds Habitat components for Windows

.DESCRIPTION
This script builds habitat components, ensure that all necesary prerequisites
are installed, and create hart packages for a windows hab binary.
#>

param (
    # The path to the component to be built. If not specified the current directory is used.
    [string]$Path=".",
    # When specified, all necessary prerequisites will be installed.
    [switch]$Configure,
    # When specified, a cargo clean will be invoked.
    [switch]$Clean,
    # When specified, cargo test will be invoked for the specified component.
    [switch]$Test,
    # When specified a .hart package of the component binary will be created.
    [switch]$Package,
    # When specified a build will not be run.
    [switch]$SkipBuild,
    # Use a optimized release build
    [switch]$Release
)

if(!$env:ChocolateyInstall) { $env:ChocolateyInstall = "$env:ProgramData\Chocolatey" }
# Set Environment Variables for the build
$ChocolateyHabitatLibDir = "$env:ChocolateyInstall\lib\habitat_native_dependencies\builds\lib"
$ChocolateyHabitatIncludeDir = "$env:ChocolateyInstall\lib\habitat_native_dependencies\builds\include"
$ChocolateyHabitatBinDir = "C:\ProgramData\chocolatey\lib\habitat_native_dependencies\builds\bin"

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
    else {
        $StartingPath
    }
}

function Test-AppVeyor {
    (test-path env:\APPVEYOR) -and ([bool]::Parse($env:APPVEYOR))
} 

function Test-RustUp {
    (get-command -Name rustup.exe -ErrorAction SilentlyContinue) -ne $null
}

function Invoke-Configure {
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
    if ((choco list habitat_native_dependencies --local-only) -match '^1 packages installed\.$') {
        choco upgrade habitat_native_dependencies --confirm -s https://www.myget.org/F/habitat/api/v2  --allowemptychecksums
    } 
    else {
        choco install habitat_native_dependencies --confirm -s https://www.myget.org/F/habitat/api/v2  --allowemptychecksums
    }

    if(!(Get-Command git -ErrorAction SilentlyContinue)) {
        choco install git --confirm
        $env:path = New-PathString -StartingPath $env:path -Path "c:\Program Files\git\cmd"
    }
    choco install libzmq_vc120 --version 4.2.3 --confirm -s https://www.nuget.org/api/v2/ --allowemptychecksums
    Copy-Item $env:ChocolateyInstall\lib\libzmq_vc120\build\native\bin\libzmq-x64-v120-mt-4_2_3_0.imp.lib $ChocolateyHabitatLibDir\zmq.lib -Force
    Copy-Item $env:ChocolateyInstall\lib\libzmq_vc120\build\native\bin\libzmq-x64-v120-mt-4_2_3_0.dll $ChocolateyHabitatBinDir\libzmq.dll -Force

    if (-not (Test-AppVeyor)) {
        # We need the Visual C 2013 Runtime for the Win32 ABI Rust
        choco install 'vcredist2013' --confirm --allowemptychecksum

        # We need the Visual C++ tools to build Rust crates (provides a compiler and linker) 
        choco install 'visualcppbuildtools' --version '14.0.25123' --confirm --allowemptychecksum

        choco install 7zip --version '16.02.0.20160811' --confirm
    }


    # Install Rust Nightly (since there aren't MSVC nightly cargo builds)
    if (Test-RustUp) {
        rustup install stable-x86_64-pc-windows-msvc
    }
    else {
        $env:PATH = New-PathString -StartingPath $env:PATH -Path "C:\Program Files\Rust stable MSVC 1.13\bin"
        if (-not (get-command rustc -ErrorAction SilentlyContinue)) {
            write-host "installing rust"
            Invoke-WebRequest -UseBasicParsing -Uri 'https://static.rust-lang.org/dist/rust-1.13.0-x86_64-pc-windows-msvc.msi' -OutFile "$env:TEMP/rust-13-stable.msi"
            start-process -filepath MSIExec.exe -argumentlist "/qn", "/i", "$env:TEMP\rust-13-stable.msi" -Wait
            $env:PATH = New-PathString -StartingPath $env:PATH -Path "C:\Program Files\Rust stable MSVC 1.13\bin"
            while (-not (get-command cargo -ErrorAction SilentlyContinue)) {
                Write-Warning "`tWaiting for `cargo` to be available."
                start-sleep -Seconds 1
            }
        }
        else {
            # TODO: version checking logic and upgrades
        }
    }
}

function Get-RustcCommand {
    if(Test-RustUp) {
        'rustup run stable-x86_64-pc-windows-msvc rustc'
    }
    else {
        'rustc'
    }
}

function Get-CargoCommand {
    if(Test-RustUp) {
        'rustup run stable-x86_64-pc-windows-msvc cargo'
    }
    else {
        'cargo'
    }
}

function Write-RustToolVersion {
    Write-Host ""
    Invoke-Expression "$(Get-RustcCommand) --version"
    Invoke-Expression "$(Get-CargoCommand) --version"  
    Write-Host ""
}

function Invoke-Build([string]$Path, [switch]$Clean, [switch]$Release) {
    $Path = Resolve-Path $Path

    $cargo = Get-CargoCommand

    Push-Location "$Path"
    if($Clean) { invoke-expression "$cargo clean" }
    Invoke-Expression "$cargo build $(if ($Release) { '--release' })" -ErrorAction Stop
    Pop-Location
}

function Invoke-Test([string]$Path, [switch]$Clean, [switch]$Release) {
    $Path = Resolve-Path $Path

    $cargo = Get-CargoCommand

    Push-Location "$Path"
    if($Clean) { invoke-expression "$cargo clean" }
    Invoke-Expression "$cargo test $(if ($Release) { '--release' })" -ErrorAction Stop
    Pop-Location
}

function New-HartPackage {
    if((Split-Path $Path -leaf) -ne "hab") {
        Invoke-Build $(Join-Path $psscriptroot 'components/hab') -Clean -Release
    }

    # Import origin key
    if (!(Test-Path "/hab/cache/keys/core-*.sig.key")) {
        if(!$env:ORIGIN_KEY) {
           throw "You do not have the core origin key imported on this machine. Please ensure the key is exported to the ORIGIN_KEY environment variable."
        }
        $env:ORIGIN_KEY | & "$psscriptroot\target\Release\hab.exe" origin key import
    }

    # Create the archive
    $pkgRoot = "$psscriptroot/results"
    New-Item -ItemType Directory -Path $pkgRoot -ErrorAction SilentlyContinue -Force

    $pkgName = 'hab'
    $pkgOrigin = 'core'
    $pkgRelease = (Get-Date).ToString('yyyyMMddhhmmss')
    $pkgVersion = (Get-Content -Path "$psscriptroot\VERSION" | Out-String).Trim()
    $pkgArtifact = "$pkgRoot/$pkgOrigin-$pkgName-$pkgVersion-$pkgRelease-x86_64-windows"
    $pkgFiles = @(
        "$psscriptroot\target\Release\hab.exe",
        'C:\Windows\System32\vcruntime140.dll',
        'C:\ProgramData\chocolatey\lib\habitat_native_dependencies\builds\bin\*.dll'
    )
    $pkgTempDir = "./hab/pkgs/$pkgOrigin/$pkgName/$pkgVersion/$pkgRelease" 
    $pkgBinDir =  "$pkgTempDir/bin"
    mkdir $pkgBinDir -Force | Out-Null
    Copy-Item $pkgFiles -Destination $pkgBinDir
    "$pkgOrigin/$pkgName/$pkgVersion/$pkgRelease" | out-file "$pkgTempDir/IDENT" -Encoding ascii
    "" | out-file "$pkgTempDir/BUILD_DEPS" -Encoding ascii
    "" | out-file "$pkgTempDir/BUILD_TDEPS" -Encoding ascii
    "" | out-file "$pkgTempDir/FILES" -Encoding ascii
    "" | out-file "$pkgTempDir/MANIFEST" -Encoding ascii
    "/hab/pkgs/$pkgOrigin/$pkgName/$pkgVersion/$pkgRelease" | out-file "$pkgTempDir/PATH" -Encoding ascii
    "" | out-file "$pkgTempDir/SVC_GROUP" -Encoding ascii
    "" | out-file "$pkgTempDir/SVC_USER" -Encoding ascii
    "x86_64-windows" | out-file "$pkgTempDir/TARGET" -Encoding ascii
    7z.exe a -ttar "$pkgArtifact.tar" ./hab
    7z.exe a -txz "$pkgArtifact.tar.xz" "$pkgArtifact.tar"

    & "$psscriptroot\target\Release\hab.exe" pkg sign --origin $pkgOrigin "$pkgArtifact.tar.xz" "$pkgArtifact.hart"
    rm "$pkgArtifact.tar", "$pkgArtifact.tar.xz", "./hab" -Recurse -force
}

if($Configure) { Invoke-Configure }

# Set Default Environmental Variables for Native Compilation
# AppVeyor will have these set already.
if (-not (Test-AppVeyor)) {
    $env:LIB = 'C:\Program Files (x86)\Microsoft Visual Studio 14.0\VC\LIB\amd64;C:\Program Files (x86)\Microsoft Visual Studio 14.0\VC\ATLMFC\LIB\amd64;C:\Program Files (x86)\Windows Kits\10\lib\10.0.10240.0\ucrt\x64;C:\Program Files (x86)\Windows Kits\NETFXSDK\4.6.1\lib\um\x64;C:\Program Files (x86)\Windows Kits\10\lib\10.0.10240.0\um\x64;'
    $env:INCLUDE = 'C:\Program Files (x86)\Microsoft Visual Studio 14.0\VC\INCLUDE;C:\Program Files (x86)\Microsoft Visual Studio 14.0\VC\ATLMFC\INCLUDE;C:\Program Files (x86)\Windows Kits\10\include\10.0.10240.0\ucrt;C:\Program Files (x86)\Windows Kits\NETFXSDK\4.6.1\include\um;C:\Program Files (x86)\Windows Kits\10\include\10.0.10240.0\shared;C:\Program Files (x86)\Windows Kits\10\include\10.0.10240.0\um;C:\Program Files (x86)\Windows Kits\10\include\10.0.10240.0\winrt;'
    $env:PATH = New-PathString -StartingPath $env:PATH -Path 'C:\Program Files (x86)\MSBuild\14.0\bin\amd64;C:\Program Files (x86)\Microsoft Visual Studio 14.0\VC\BIN\amd64;C:\Program Files (x86)\Microsoft Visual Studio 14.0\VC\VCPackages;C:\WINDOWS\Microsoft.NET\Framework64\v4.0.30319;C:\WINDOWS\Microsoft.NET\Framework64\;C:\Program Files (x86)\Windows Kits\10\bin\x64;C:\Program Files (x86)\Windows Kits\10\bin\x86;C:\Program Files (x86)\Microsoft SDKs\Windows\v10.0A\bin\NETFX 4.6.1 Tools\x64\'
}

$env:PATH                       = New-PathString -StartingPath $env:PATH    -Path 'C:\Program Files\7-Zip'
$env:PATH                       = New-PathString -StartingPath $env:PATH    -Path $ChocolateyHabitatBinDir
$env:LIB                        = New-PathString -StartingPath $env:LIB     -Path $ChocolateyHabitatLibDir
$env:INCLUDE                    = New-PathString -StartingPath $env:INCLUDE -Path $ChocolateyHabitatIncludeDir
$env:SODIUM_LIB_DIR             = $ChocolateyHabitatLibDir
$env:LIBARCHIVE_INCLUDE_DIR     = $ChocolateyHabitatIncludeDir
$env:LIBARCHIVE_LIB_DIR         = $ChocolateyHabitatLibDir
$env:OPENSSL_LIBS               = 'ssleay32:libeay32'
$env:OPENSSL_LIB_DIR            = $ChocolateyHabitatLibDir
$env:OPENSSL_INCLUDE_DIR        = $ChocolateyHabitatIncludeDir
$env:LIBZMQ_PREFIX              = Split-Path $ChocolateyHabitatLibDir -Parent

Write-RustToolVersion

if ($Test) { Invoke-Test $Path -Clean:$Clean -Release:$Release }
if (!$SkipBuild) { Invoke-Build $Path -Clean:$Cean -Release:$Release }
if($Package) { New-HartPackage }

exit $LASTEXITCODE
