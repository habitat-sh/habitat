$pkg_name = "hab"
$pkg_origin = "core"
$pkg_version = "$(Get-Content $PLAN_CONTEXT/../../VERSION)"
$pkg_maintainer = "The Habitat Maintainers <humans@habitat.sh>"
$pkg_license = @("Apache-2.0")
$pkg_source = "nosuchfile.tar.gz"
$pkg_bin_dirs = @("bin")

# No runtime or build dependencies yet
$pkg_deps = @()
$pkg_build_deps = @()

function _Invoke-CommonPrepare {
    Invoke-DefaultPrepare

    # Can be either `--release` or `--debug` to determine cargo build strategy
    $script:build_type = "--release"
    Write-BuildLine "Building artifacts with $($build_type.Substring(2))"

    # Used by the `build.rs` program to set the version of the binaries
    $env:PLAN_VERSION = "${pkg_version}/${pkg_release}"
    Write-BuildLine "Setting PLAN_VERSION=$env:PLAN_VERSION"

    # Used by Cargo to use a pristine, isolated directory for all compilation
    $env:CARGO_TARGET_DIR = "$HAB_CACHE_SRC_PATH\$pkg_dirname"
    Write-BuildLine "Setting CARGO_TARGET_DIR=$env:CARGO_TARGET_DIR"
}

function Invoke-Prepare {
    _Invoke-CommonPrepare

    $script:rustc_target = "x86_64-pc-windows-gnu"
    Write-BuildLine "Setting rustc_target=$rustc_target"
}

function Invoke-Build {
    Push-Location "$PLAN_CONTEXT"
#    cargo build $build_type.Replace("--debug", "") --target=$rustc_target --verbose
    Pop-Location
}
