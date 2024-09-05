[Diagnostics.CodeAnalysis.SuppressMessage("PSUseApprovedVerbs", '', Scope="function")]
param()
function Load-Scaffolding {
    $pkg_deps += @("$env:HAB_ORIGIN/minimal_package")
    $pkg_build_deps += @("$env:HAB_ORIGIN/dep-pkg-1")
}
