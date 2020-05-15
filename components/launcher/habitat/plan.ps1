$pkg_name = "hab-launcher"
$pkg_origin = "core"
$pkg_maintainer = "The Habitat Maintainers <humans@habitat.sh>"
$pkg_license = @("Apache-2.0")
$pkg_deps=@(
    "core/openssl",
    "core/zlib",
    "core/xz"
)
$pkg_bin_dirs = @("bin")
$pkg_build_deps = @(
    "core/visual-cpp-redist-2015",
    "core/visual-cpp-build-tools-2015",
    "core/rust/$(Get-Content "$PLAN_CONTEXT/../../../rust-toolchain")",
    "core/cacerts",
    "core/git"
)

function Invoke-Prepare {
    if($env:HAB_CARGO_TARGET_DIR) {
        $env:CARGO_TARGET_DIR           = "$env:HAB_CARGO_TARGET_DIR"
    } else {
        $env:CARGO_TARGET_DIR           = "$HAB_CACHE_SRC_PATH/$pkg_dirname"
    }

    $env:SSL_CERT_FILE              = "$(Get-HabPackagePath "cacerts")/ssl/certs/cacert.pem"
    $env:PLAN_VERSION               = "$pkg_version/$pkg_release"
    $env:LIB                        += ";$HAB_CACHE_SRC_PATH/$pkg_dirname/lib"
    $env:INCLUDE                    += ";$HAB_CACHE_SRC_PATH/$pkg_dirname/include"
    $env:OPENSSL_LIBS               = 'ssleay32:libeay32'
    $env:OPENSSL_LIB_DIR            = "$(Get-HabPackagePath "openssl")/lib"
    $env:OPENSSL_INCLUDE_DIR        = "$(Get-HabPackagePath "openssl")/include"
}

function pkg_version {
    git rev-list (git rev-parse HEAD) --count
}

function Invoke-Before {
    Set-PkgVersion
    $script:pkg_dirname = "${pkg_name}-${pkg_version}"
    $script:pkg_prefix = "$HAB_PKG_PATH\$pkg_origin\$pkg_name\$pkg_version\$pkg_release"
    $script:pkg_artifact="$HAB_CACHE_ARTIFACT_PATH\${pkg_origin}-${pkg_name}-${pkg_version}-${pkg_release}-${pkg_target}.${_artifact_ext}"
}

function Invoke-Build {
    Push-Location "$PLAN_CONTEXT/.."
    try {
        cargo build --release
        if($LASTEXITCODE -ne 0) {
            Write-Error "Cargo build failed!"
        }
    } finally { Pop-Location }
}

function Invoke-Install {
    Copy-Item "$env:CARGO_TARGET_DIR/release/hab-launch.exe" "$pkg_prefix/bin/hab-launch.exe"
    Copy-Item "$(Get-HabPackagePath "openssl")/bin/*.dll" "$pkg_prefix/bin"
    Copy-Item "$(Get-HabPackagePath "zlib")/bin/*.dll" "$pkg_prefix/bin"
    Copy-Item "$(Get-HabPackagePath "xz")/bin/*.dll" "$pkg_prefix/bin"
    Copy-Item "$(Get-HabPackagePath "visual-cpp-redist-2015")/bin/*" "$pkg_prefix/bin"
}
