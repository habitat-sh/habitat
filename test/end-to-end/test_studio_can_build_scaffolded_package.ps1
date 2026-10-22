Describe "package using scaffolding" {
    BeforeAll {
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

        hab origin key generate $env:HAB_ORIGIN
        $script:minimal = Invoke-PlanBuild minimal-package
        $depPkg = Invoke-PlanBuild dep-pkg-1
        if ($IsLinux) {
            hab studio run "rm -rf /hab/pkgs/$($depPkg.Ident)/hooks"
        }
        $script:scaffolding = Invoke-PlanBuild scaffolding
        $script:consumer = Invoke-PlanBuild use_scaffolding
    }

    It "inherits scaffolding dependencies" {
        hab pkg install "results/$($script:minimal.Artifact)"
        hab pkg install "results/$($depPkg.Artifact)"
        Remove-Item "/hab/pkgs/$($depPkg.Ident)/hooks" -Recurse -Force
        hab pkg install "results/$($script:scaffolding.Artifact)"
        hab pkg install "results/$($script:consumer.Artifact)"
        # scaffolding has minimal_package as runtime and dep-pkg-1 as build time deps

        "/hab/pkgs/$($script:consumer.Ident)/DEPS" | Should -FileContentMatch "$env:HAB_ORIGIN/minimal_package"
        "/hab/pkgs/$($script:consumer.Ident)/BUILD_DEPS" | Should -FileContentMatch "$env:HAB_ORIGIN/dep-pkg-1"
    }
}
