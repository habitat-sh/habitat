. $PSScriptRoot\..\shared.ps1

function Install-HabPkg([string[]]$idents) {
  $idents | % {
      $id = $_
      $installedPkgs=hab pkg list $id | ? { $_.StartsWith($id)}

      if($installedPkgs){
          Write-host "$id already installed"
      } else {
          hab pkg install $id
      }
  }
}

function Setup-Environment {
  $env:HAB_LICENSE = "accept-no-persist"
  Install-Habitat

  Install-HabPkg @(
      "core/cacerts",
      "core/libarchive",
      "core/libsodium",
      "core/openssl",
      "core/protobuf",
      "core/visual-cpp-build-tools-2015",
      "core/xz",
      "core/zeromq",
      "core/zlib"
  )

  # Set up some path variables for ease of use later
  $cacertsDir     = & hab pkg path core/cacerts
  $libarchiveDir  = & hab pkg path core/libarchive
  $libsodiumDir   = & hab pkg path core/libsodium
  $opensslDir     = & hab pkg path core/openssl
  $protobufDir    = & hab pkg path core/protobuf
  $xzDir          = & hab pkg path core/xz
  $zeromqDir      = & hab pkg path core/zeromq
  $zlibDir        = & hab pkg path core/zlib

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
  $env:PATH                       = New-PathString -StartingPath $env:PATH -Path "$protobufDir\bin;$zeromqDir\bin;$libarchiveDir\bin;$libsodiumDir\bin;$zlibDir\bin;$xzDir\bin;$opensslDir\bin"

  $vsDir = & hab pkg path core/visual-cpp-build-tools-2015
  $env:LIB = (Get-Content "$vsDir\LIB_DIRS")
  $env:INCLUDE = (Get-Content "$vsDir\INCLUDE_DIRS")
  $env:PATH = New-PathString -StartingPath $env:PATH -Path (Get-Content "$vsDir\PATH")
}

function Get-NightlyToolchain {
  Get-Content "$PSScriptRoot\..\..\..\RUST_NIGHTLY_VERSION"
}
