Describe "Supervisor binds" {
    BeforeAll {
        hab origin key generate $env:HAB_ORIGIN

        Invoke-BuildAndInstall testpkgbindproducer
        Invoke-BuildAndInstall testpkgbindconsumer
        $supLog = New-SupervisorLogFile("test_supervisor_binds")
        Start-Supervisor -LogFile $supLog -Timeout 45 | Out-Null
    }

    It "consumer bind to producer export" {
        Load-SupervisorService -PackageName $env:HAB_ORIGIN/testpkgbindproducer
        Load-SupervisorService -PackageName $env:HAB_ORIGIN/testpkgbindconsumer -Bind alias:testpkgbindproducer.default

        # The consumer's myconfig.conf is a template that holds the value
        # of the producers exported property which should be "default1"
        Get-Content "/hab/svc/testpkgbindconsumer/config/myconfig.conf" | Should -Be "default1"
    }
}
