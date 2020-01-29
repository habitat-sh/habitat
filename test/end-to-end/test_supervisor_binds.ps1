Function Invoke-BuildAndInstall($PackageName) {
    hab pkg build test/fixtures/$PackageName
    if($IsLinux) {
        Get-Content "results/last_build.env" | ForEach-Object { Add-Content "results/last_build.ps1" -Value "`$$($_.Replace("=", '="'))`"" }
    }
    . ./results/last_build.ps1
    hab pkg install ./results/$pkg_artifact
}

Describe "Supervisor binds" {
    BeforeAll {
        hab origin key generate $env:HAB_ORIGIN

        Invoke-BuildAndInstall testpkgbindproducer
        Invoke-BuildAndInstall testpkgbindconsumer

        Start-Supervisor -Timeout 45 | Out-Null
    }

    It "consumer bind to producer export" {
        Load-SupervisorService -PackageName $env:HAB_ORIGIN/testpkgbindproducer -Timeout 20
        Load-SupervisorService -PackageName $env:HAB_ORIGIN/testpkgbindconsumer -Timeout 20 -Bind alias:testpkgbindproducer.default

        # The consumer's myconfig.conf is a template that holds the value
        # of the producers exported property which should be "default1"
        Get-Content "/hab/svc/testpkgbindconsumer/config/myconfig.conf" | Should -Be "default1"
    }
}
