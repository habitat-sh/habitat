hab origin key generate $env:HAB_ORIGIN

Function Invoke-PlanBuild($package) {
    hab pkg build test/fixtures/$package -R | Out-Null
    if ($IsLinux) {
        # This changes the format of last_build from `var=value` to `$var='value'`
        # so that powershell can parse and source the script
        Set-Content -Path "results/last_build.ps1" -Value ""
        Get-Content "results/last_build.env" | ForEach-Object { Add-Content "results/last_build.ps1" -Value "`$$($_.Replace("=", '="'))`"" }
    }
    . results/last_build.ps1
    @{ Artifact = $pkg_artifact; Ident = $pkg_ident }
}

Describe "package using scaffolding" {
    $minimal = Invoke-PlanBuild minimal-package
    $depPkg = Invoke-PlanBuild dep-pkg-1
    $scaffolding = Invoke-PlanBuild scaffolding
    $consumer = Invoke-PlanBuild use_scaffolding
    It "inherits scaffolding dependencies" {
        hab pkg install "results/$($minimal.Artifact)"
        hab pkg install "results/$($depPkg.Artifact)"
        hab pkg install "results/$($scaffolding.Artifact)"
        hab pkg install "results/$($consumer.Artifact)"
        # scaffolding has minimal_package as runtime and dep-pkg-1 as build time deps

        "/hab/pkgs/$($consumer.Ident)/DEPS" | Should -FileContentMatch "$env:HAB_ORIGIN/minimal_package"
        "/hab/pkgs/$($consumer.Ident)/BUILD_DEPS" | Should -FileContentMatch "$env:HAB_ORIGIN/dep-pkg-1"
    }
}