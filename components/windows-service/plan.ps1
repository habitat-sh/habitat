$pkg_name="windows-service"
$pkg_origin="core"
$pkg_version="0.7.0"
$pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
$pkg_license=@('Apache-2.0')
$pkg_description="A Windows Service for runnung the Habitat Supervisor"
$pkg_deps=@(
    "core/hab-launcher",
    "core/powershell/$(Get-Content "$PLAN_CONTEXT/../../POWERSHELL_VERSION")"
)
$pkg_build_deps=@("core/dotnet-core-sdk/8.0.303/20250708090906")
$pkg_bin_dirs=@("bin")

function Invoke-Build {
    Copy-Item $PLAN_CONTEXT/* $HAB_CACHE_SRC_PATH/$pkg_dirname -Force
    dotnet restore $HAB_CACHE_SRC_PATH/$pkg_dirname/WindowsService.csproj
    dotnet build $HAB_CACHE_SRC_PATH/$pkg_dirname/WindowsService.csproj -r win-x64 -p:Version=$pkg_version
    if($LASTEXITCODE -ne 0) {
        Write-Error "dotnet build failed!"
    }
}

function Invoke-Install {
    dotnet publish $HAB_CACHE_SRC_PATH/$pkg_dirname/WindowsService.csproj -r win-x64 --output "$pkg_prefix/bin" -p:Version=$pkg_version
}
