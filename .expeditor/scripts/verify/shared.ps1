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
        "core/protobuf",
        "core/visual-cpp-build-tools-2015",
        "core/zeromq"
    )

    # Set up some path variables for ease of use later
    $cacertsDir     = & hab pkg path core/cacerts
    $protobufDir    = & hab pkg path core/protobuf
    $zeromqDir      = & hab pkg path core/zeromq

    # Set some required variables
    $env:LIBZMQ_PREFIX              = "$zeromqDir"
    $env:SSL_CERT_FILE              = "$cacertsDir\ssl\certs\cacert.pem"
    $env:LD_LIBRARY_PATH            = "$env:LIBZMQ_PREFIX\lib;$env:SODIUM_LIB_DIR"
    $env:PATH                       = New-PathString -StartingPath $env:PATH -Path "$protobufDir\bin;$zeromqDir\bin"

    $vsDir = & hab pkg path core/visual-cpp-build-tools-2015
    $env:LIB = "$(Get-Content "$vsDir\LIB_DIRS");$env:LIBZMQ_PREFIX\lib"
    $env:INCLUDE = (Get-Content "$vsDir\INCLUDE_DIRS")
    $env:PATH = New-PathString -StartingPath $env:PATH -Path (Get-Content "$vsDir\PATH")
}

function Get-NightlyToolchain {
    "$(Get-Content $PSScriptRoot\..\..\..\RUST_NIGHTLY_VERSION)-x86_64-pc-windows-msvc"
}
