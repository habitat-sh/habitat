. $PSScriptRoot\..\shared.ps1

function Install-HabPkg([string[]]$idents) {
    $idents | ForEach-Object {
        $id = $_
        $installedPkgs=hab pkg list $id | Where-Object { $_.StartsWith($id)}

        if($installedPkgs){
            Write-Host "$id already installed"
        } else {
            hab pkg install $id
        }
    }
}

function Initialize-Environment {
    $env:HAB_LICENSE = "accept-no-persist"
    Install-Habitat

    Install-HabPkg @(
        "core/cacerts",
        "core/openssl",
        "core/protobuf",
        "core/visual-cpp-build-tools-2015",
        "core/zeromq",
        "core/zlib"
    )

    # Set up some path variables for ease of use later
    $cacertsDir     = & hab pkg path core/cacerts
    $opensslDir     = & hab pkg path core/openssl
    $protobufDir    = & hab pkg path core/protobuf
    $zeromqDir      = & hab pkg path core/zeromq
    $zlibDir        = & hab pkg path core/zlib

    # Set some required variables
    $env:OPENSSL_LIBS               = 'ssleay32:libeay32'
    $env:OPENSSL_LIB_DIR            = "$opensslDir\lib"
    $env:OPENSSL_INCLUDE_DIR        = "$opensslDir\include"
    $env:LIBZMQ_PREFIX              = "$zeromqDir"
    $env:SSL_CERT_FILE              = "$cacertsDir\ssl\certs\cacert.pem"
    $env:OPENSSL_STATIC             = "true"
    $env:LD_LIBRARY_PATH            = "$env:LIBZMQ_PREFIX\lib;$env:SODIUM_LIB_DIR;$zlibDir\lib"
    $env:PATH                       = New-PathString -StartingPath $env:PATH -Path "$protobufDir\bin;$zeromqDir\bin;$zlibDir\bin;$opensslDir\bin"

    $vsDir = & hab pkg path core/visual-cpp-build-tools-2015
    $env:LIB = "$(Get-Content "$vsDir\LIB_DIRS");$env:LIBZMQ_PREFIX\lib"
    $env:INCLUDE = (Get-Content "$vsDir\INCLUDE_DIRS")
    $env:PATH = New-PathString -StartingPath $env:PATH -Path (Get-Content "$vsDir\PATH")
}

function Get-NightlyToolchain {
    "$(Get-Content $PSScriptRoot\..\..\..\RUST_NIGHTLY_VERSION)-x86_64-pc-windows-msvc"
}
