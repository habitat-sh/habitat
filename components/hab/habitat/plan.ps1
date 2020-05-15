$pkg_name = "hab"
$pkg_origin = "core"
$pkg_maintainer = "The Habitat Maintainers <humans@habitat.sh>"
$pkg_license = @("Apache-2.0")
$pkg_deps=@(
    "core/openssl",
    "core/visual-cpp-redist-2015"
)
$pkg_bin_dirs = @("bin")
$pkg_build_deps = @(
    "core/visual-cpp-build-tools-2015",
    "core/rust/$(Get-Content "$PLAN_CONTEXT/../../../rust-toolchain")",
    "core/cacerts"
)

function pkg_version {
    Get-Content "$SRC_PATH/VERSION"
}

function Invoke-Before {
    Invoke-DefaultBefore
    Set-PkgVersion
}

function Invoke-Prepare {
    if($env:HAB_CARGO_TARGET_DIR) {
        $env:CARGO_TARGET_DIR           = "$env:HAB_CARGO_TARGET_DIR"
    } else {
        $env:CARGO_TARGET_DIR           = "$HAB_CACHE_SRC_PATH/$pkg_dirname"
    }

    $env:SSL_CERT_FILE              = "$(Get-HabPackagePath "cacerts")/ssl/certs/cacert.pem"
    $env:LIB                        += ";$HAB_CACHE_SRC_PATH/$pkg_dirname/lib"
    $env:INCLUDE                    += ";$HAB_CACHE_SRC_PATH/$pkg_dirname/include"
    $env:OPENSSL_LIBS               = 'ssleay32:libeay32'
    $env:OPENSSL_LIB_DIR            = "$(Get-HabPackagePath "openssl")/lib"
    $env:OPENSSL_INCLUDE_DIR        = "$(Get-HabPackagePath "openssl")/include"

    # Used by the `build.rs` program to set the version of the binaries
    $env:PLAN_VERSION = "$pkg_version/$pkg_release"
    Write-BuildLine "Setting env:PLAN_VERSION=$env:PLAN_VERSION"

    # Used to set the active package target for the binaries at build time
    $env:PLAN_PACKAGE_TARGET = "$pkg_target"
    Write-BuildLine "Setting env:PLAN_PACKAGE_TARGET=$env:PLAN_PACKAGE_TARGET"
}

function Invoke-Build {
    Push-Location "$PLAN_CONTEXT"
    try {
        cargo build --release --verbose
        if($LASTEXITCODE -ne 0) {
            Write-Error "Cargo build failed!"
        }
    } finally { Pop-Location }
}

function Invoke-Install {
    Write-BuildLine "$HAB_CACHE_SRC_PATH/$pkg_dirname"
    Copy-Item "$env:CARGO_TARGET_DIR/release/hab.exe" "$pkg_prefix/bin/hab.exe"
    Copy-Item "$(Get-HabPackagePath "openssl")/bin/*.dll" "$pkg_prefix/bin"
    Copy-Item "$(Get-HabPackagePath "visual-cpp-redist-2015")/bin/*.dll" "$pkg_prefix/bin"
}

function Invoke-Clean {
    if(!$env:HAB_SKIP_CLEAN) { Invoke-DefaultClean }
}
