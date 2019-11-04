$pkg_name = "hab-plan-build-ps1"
$pkg_origin = "core"
$pkg_maintainer = "The Habitat Maintainers <humans@habitat.sh>"
$pkg_license = @("Apache-2.0")
$pkg_bin_dirs = @("bin")

# No runtime or build dependencies yet
$pkg_deps = @()
$pkg_build_deps = @()

$bin = "hab-plan-build.ps1"

function pkg_version {
    Get-Content "$SRC_PATH/VERSION"
}

function Invoke-Before {
  Invoke-DefaultBefore
  Set-PkgVersion
}

function Invoke-Build {
    # Embed the release version of the program.
    (Get-Content "$PLAN_CONTEXT\..\bin\${bin}" -Encoding Ascii) | ForEach-Object {
      $_.replace(
        "@VERSION@",
        "$pkg_version/$pkg_release"
      ).
      replace(
        "`$script:pkg_target = `"@@pkg_target@@`"",
        "`$script:pkg_target = `"$pkg_target`""
      )
    } | Out-File "$bin" -Encoding ascii
}

function Invoke-Install {
    New-Item "$pkg_prefix\bin" -ItemType Directory -Force | Out-Null
    Copy-Item "$PLAN_CONTEXT\..\bin\*" "$pkg_prefix\bin" -Force
    Copy-Item "$bin" "$pkg_prefix\bin\$bin" -Force
}
