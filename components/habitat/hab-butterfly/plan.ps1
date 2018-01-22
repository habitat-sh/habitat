$pkg_name = "hab-butterfly"
$pkg_origin = "core"
$pkg_version = "$(Get-Content $PLAN_CONTEXT/../../VERSION)"
$pkg_maintainer = "The Habitat Maintainers <humans@habitat.sh>"
$pkg_license = @("Apache-2.0")
$pkg_source = "https://s3-us-west-2.amazonaws.com/habitat-win-deps/hab-win-deps.zip"
$pkg_shasum="00b34fb983ebc43bfff9e8e2220d23db200cb45494a4971a5e2e733f1d73d04b"
$pkg_bin_dirs = @("bin")
$pkg_build_deps = @("core/visual-cpp-redist-2013", "core/rust", "core/cacerts")

function Invoke-Prepare {
    if($env:HAB_CARGO_TARGET_DIR) {
        $env:CARGO_TARGET_DIR           = "$env:HAB_CARGO_TARGET_DIR"
    }
    else {
        $env:CARGO_TARGET_DIR           = "$env:HAB_CACHE_SRC_PATH/$pkg_dirname"
    }

    $env:SSL_CERT_FILE              = "$(Get-HabPackagePath "cacerts")/ssl/certs/cacert.pem"
    $env:PLAN_VERSION               = "$pkg_version/$pkg_release"
    $env:LIB                        += ";$HAB_CACHE_SRC_PATH/$pkg_dirname/lib"
    $env:INCLUDE                    += ";$HAB_CACHE_SRC_PATH/$pkg_dirname/include"
    $env:SODIUM_LIB_DIR             = "$HAB_CACHE_SRC_PATH/$pkg_dirname/lib"
    $env:LIBARCHIVE_INCLUDE_DIR     = "$HAB_CACHE_SRC_PATH/$pkg_dirname/include"
    $env:LIBARCHIVE_LIB_DIR         = "$HAB_CACHE_SRC_PATH/$pkg_dirname/lib"
    $env:OPENSSL_LIBS               = 'ssleay32:libeay32'
    $env:OPENSSL_LIB_DIR            = "$HAB_CACHE_SRC_PATH/$pkg_dirname/lib"
    $env:OPENSSL_INCLUDE_DIR        = "$HAB_CACHE_SRC_PATH/$pkg_dirname/include"
    $env:LIBZMQ_PREFIX              = "$HAB_CACHE_SRC_PATH/$pkg_dirname"
}

function Invoke-Unpack {
  Expand-Archive -Path "$HAB_CACHE_SRC_PATH/hab-win-deps.zip" -DestinationPath "$HAB_CACHE_SRC_PATH/$pkg_dirname"
}

function Invoke-Build {
    Push-Location "$PLAN_CONTEXT"
    try {
        cargo build --release
        if($LASTEXITCODE -ne 0) {
            Write-Error "Cargo build failed!"
        }
    }
    finally { Pop-Location }
}

function Invoke-Install {
    Copy-Item "$env:CARGO_TARGET_DIR/release/hab-butterfly.exe" "$pkg_prefix/bin/hab-butterfly.exe"
    Copy-Item "$HAB_CACHE_SRC_PATH/$pkg_dirname/bin/*" "$pkg_prefix/bin"
    Copy-Item "$(Get-HabPackagePath "visual-cpp-redist-2013")/bin/*" "$pkg_prefix/bin"
}
