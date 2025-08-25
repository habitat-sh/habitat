$Env:HAB_FEAT_SERVICE_CONFIG_FILES = "1"
$configPath = "/hab/sup/default/config"
$supConfig = "/hab/sup/default/config/sup.toml"
$supBadConfig = "/hab/sup/default/config/bad_sup.toml"
$pkgName = "core/nginx"
$svcName = ($pkgName -split "/")[1]
$svcConfig = "/hab/sup/default/config/svc.toml"
$svcDirConfig = "/hab/sup/default/config/svc"
$mySvcConfig = "$svcDirConfig/my_svc.toml"

Describe "reading from supervisor and service config files" {
    Remove-Item -Force -Recurse -ErrorAction Ignore $supConfig
    Remove-Item -Force -Recurse -ErrorAction Ignore $svcConfig
    Remove-Item -Force -Recurse -ErrorAction Ignore $svcDirConfig
    New-Item -ItemType directory -Force -Path $configPath
    New-Item -ItemType directory -Force -Path $svcDirConfig

    It "Supervisor starts with output of 'hab sup run --generate-config'" {
        # Run this twice. The first time the output is polluted with installing the launcher.
        $out = hab sup run --generate-config
        $out = hab sup run --generate-config
        Set-Content -Path $supConfig -Force -Value $out
        Get-Content -Path $supConfig | Should -Contain "###  The listen address for the Gossip Gateway"
        $supLog = New-SupervisorLogFile("supervisor_starts_with_output_of_hab_sup_run_--generate-config")
        Start-Supervisor -LogFile $supLog -Timeout 45
    }

    Stop-Supervisor

    It "Supervisor starts with correctly configured gossip address" {
        $address = "127.0.0.1:1234"
        Set-Content -Path $supConfig -Force -Value "listen_gossip = '$address'"

        $supLog = New-SupervisorLogFile("supervisor_starts_with_correctly_configured_gossip_address")
        Start-Supervisor -Timeout 45 -LogFile $supLog
        Start-Sleep -Seconds 3

        $out = Get-Content -Path $supLog
        $out | Should -Contain "hab-sup(MR): Starting gossip-listener on $address"
    }

    Stop-Supervisor

    It "Supervisor does not start with invalid config file" {
        $supLog = New-SupervisorLogFile("supervisor_does_not_start_with_invalid_config_file")
        Set-Content -Path $supBadConfig -Force -Value "a bad confg"

        {
            Start-Supervisor -Timeout 3 -LogFile $supLog -SupArgs @( `
                    "--config-files=$supBadConfig" `
            )
        } | Should -Throw
    }

    Stop-Supervisor
    $supLog = New-SupervisorLogFile("test_config_files")
    Start-Supervisor -LogFile $supLog -Timeout 45

    It "service does not start without ident" {
        $out = hab svc load --generate-config
        Set-Content -Path $svcConfig -Force -Value $out
        Get-Content -Path $svcConfig | Should -Contain "# pkg_ident = "

        {
            Invoke-NativeCommand hab svc load
        } | Should -Throw
    }

    It "service does start with ident" {
        ((Get-Content -Path $svcConfig -Raw) -replace "# pkg_ident = ", "pkg_ident = '$pkgName'") | Set-Content -Path $svcConfig

        Invoke-NativeCommand hab svc load
        Wait-SupervisorService $svcName
    }

    Unload-SupervisorService -PackageName $pkgName -Timeout 20
    Remove-Item -Force -Recurse -ErrorAction Ignore $svcConfig

    It "service starts on Supervisor startup" {
        $supLog = New-SupervisorLogFile("service_starts_on_supervisor_startup")
        Start-Supervisor -LogFile $supLog
        Wait-SupervisorService $svcName
    }

    Remove-Item -Force -Recurse -ErrorAction Ignore $supConfig
    Remove-Item -Force -Recurse -ErrorAction Ignore $svcConfig
    Remove-Item -Force -Recurse -ErrorAction Ignore $svcDirConfig
    Unload-SupervisorService -PackageName $pkgName -Timeout 20
    Stop-Supervisor
}
