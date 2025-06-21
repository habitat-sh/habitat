$pkg_name = "hab-launcher"
$pkg_origin = "chef"
$pkg_maintainer = "The Habitat Maintainers <humans@habitat.sh>"
$pkg_license = @("Apache-2.0")
$pkg_deps=@()
$pkg_bin_dirs = @("bin")
$pkg_build_deps = @(
    "core/visual-cpp-redist-2022",
    "core/visual-build-tools-2022",
    "core/rust/$((ConvertFrom-StringData (Get-Content "$PLAN_CONTEXT/../../../rust-toolchain")[1]).channel.Replace('"', ''))",
    "core/cacerts",
    "core/git",
    "core/windows-11-sdk",
    "core/protobuf"
)

function Invoke-Prepare {
    if($env:HAB_CARGO_TARGET_DIR) {
        $env:CARGO_TARGET_DIR           = "$env:HAB_CARGO_TARGET_DIR"
    } else {
        $env:CARGO_TARGET_DIR           = Join-Path -Path "$HAB_CACHE_SRC_PATH" -ChildPath "$pkg_dirname"
    }

    $env:SSL_CERT_FILE              = "$(Get-HabPackagePath "cacerts")/ssl/certs/cacert.pem"
    $env:PLAN_VERSION               = "$pkg_version"
    Write-BuildLine "Setting env:PLAN_VERSION=$env:PLAN_VERSION"
    $env:LIB                        += ";$HAB_CACHE_SRC_PATH/$pkg_dirname/lib"
    $env:INCLUDE                    += ";$HAB_CACHE_SRC_PATH/$pkg_dirname/include"
    $env:PROTOC_NO_VENDOR           = 1
}

function pkg_version {
    git rev-list (git rev-parse HEAD) --count
    if($LASTEXITCODE -ne 0) {
        Write-Error "Unable to deterine version from git!"
    }
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
    Copy-Item "$(Get-HabPackagePath "visual-cpp-redist-2022")/bin/*" "$pkg_prefix/bin"
}

function Invoke-Clean {
    if(!$env:HAB_SKIP_CLEAN) { Invoke-DefaultClean }
}
