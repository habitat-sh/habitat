$pkg_name = "hab-sup"
$pkg_origin = "chef"
$pkg_maintainer = "The Habitat Maintainers <humans@habitat.sh>"
$pkg_license = @("Apache-2.0")
$pkg_bin_dirs = @("bin")
$pkg_deps=@(
    "core/powershell/$(Get-Content "$PLAN_CONTEXT/../../../POWERSHELL_VERSION")",
    "core/visual-cpp-redist-2022",
    "core/zeromq"
)
$pkg_build_deps = @(
    "core/visual-build-tools-2022",
    "core/rust/$((ConvertFrom-StringData (Get-Content "$PLAN_CONTEXT/../../../rust-toolchain")[1]).channel.Replace('"', ''))",
    "core/cacerts",
    "core/windows-11-sdk",
    "core/protobuf"
)

function pkg_version {
    Get-Content "$SRC_PATH/../../VERSION"
}

function Invoke-Before {
    Invoke-DefaultBefore
    Set-PkgVersion
}

function Invoke-Prepare {
    if($env:HAB_CARGO_TARGET_DIR) {
        $env:CARGO_TARGET_DIR           = "$env:HAB_CARGO_TARGET_DIR"
    } else {
        $env:CARGO_TARGET_DIR           = Join-Path -Path "$HAB_CACHE_SRC_PATH" -ChildPath "$pkg_dirname"
    }

    $env:SSL_CERT_FILE              = "$(Get-HabPackagePath "cacerts")/ssl/certs/cacert.pem"
    $env:LIB                        += ";$HAB_CACHE_SRC_PATH/$pkg_dirname/lib"
    $env:INCLUDE                    += ";$HAB_CACHE_SRC_PATH/$pkg_dirname/include"
    $env:LIBZMQ_PREFIX              = "$(Get-HabPackagePath "zeromq")"

    # Used by the `build.rs` program to set the version of the binaries
    $env:PLAN_VERSION = "$pkg_version/$pkg_release"
    Write-BuildLine "Setting env:PLAN_VERSION=$env:PLAN_VERSION"

    # Used to set the active package target for the binaries at build time
    $env:PLAN_PACKAGE_TARGET = "$pkg_target"
    Write-BuildLine "Setting env:PLAN_PACKAGE_TARGET=$env:PLAN_PACKAGE_TARGET"

    $env:PROTOC_NO_VENDOR=1
}

function Invoke-Build {
    Push-Location "$PLAN_CONTEXT"
    try {
        cargo build --release
        if($LASTEXITCODE -ne 0) {
            Write-Error "Cargo build failed!"
        }
    } finally { Pop-Location }
}

function Invoke-Install {
    Copy-Item -Path "$env:CARGO_TARGET_DIR/release/hab-sup.exe" -Destination "$pkg_prefix/bin/hab-sup.exe"
    Copy-Item -Path "$PLAN_CONTEXT/../static/named_pipe_service.ps1" -Destination "$pkg_prefix/bin/named_pipe_service.ps1"
    Copy-Item -Path "$(Get-HabPackagePath "zeromq")/bin/*.dll" -Destination "$pkg_prefix/bin"
    Copy-Item -Path "$(Get-HabPackagePath "visual-cpp-redist-2022")/bin/*.dll" -Destination "$pkg_prefix/bin"
    Copy-Item -Path "$SRC_PATH/../../NOTICES.txt" -Destination "$pkg_prefix"/NOTICES.txt
}

function Invoke-Clean {
    if(!$env:HAB_SKIP_CLEAN) { Invoke-DefaultClean }
}
