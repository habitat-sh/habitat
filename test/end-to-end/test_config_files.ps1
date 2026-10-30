$setup = {
    $configPath = "/hab/sup/default/config"
    $supConfig = "/hab/sup/default/config/sup.toml"
    $supBadConfig = "/hab/sup/default/config/bad_sup.toml"
}
if ($useWindowsPester5) {
    BeforeAll $setup
} else {
    . $setup
}

Describe "reading from supervisor and service config files" {
    $setup = {
        Remove-Item -Force -Recurse -ErrorAction Ignore $supConfig
        New-Item -ItemType directory -Force -Path $configPath
    }
    if ($useWindowsPester5) {
        BeforeAll $setup
        AfterEach { Stop-Supervisor }
    } else {
        . $setup
    }

    It "Supervisor starts with output of 'hab sup run --generate-config'" {
        # Run this twice. The first time the output is polluted with installing the launcher.
        $out = hab sup run --generate-config
        $out = hab sup run --generate-config
        Set-Content -Path $supConfig -Force -Value $out
        Get-Content -Path $supConfig | Should -Contain "###  The listen address for the Gossip Gateway."
        $supLog = New-SupervisorLogFile("supervisor_starts_with_output_of_hab_sup_run_--generate-config")
        Start-Supervisor -LogFile $supLog -Timeout 45
    }

    if (!$useWindowsPester5) { Stop-Supervisor }

    It "Supervisor starts with correctly configured gossip address" {
        $address = "127.0.0.1:1234"
        Set-Content -Path $supConfig -Force -Value "listen_gossip = '$address'"

        $supLog = New-SupervisorLogFile("supervisor_starts_with_correctly_configured_gossip_address")
        Start-Supervisor -Timeout 45 -LogFile $supLog
        Start-Sleep -Seconds 3

        $out = Get-Content -Path $supLog
        $out | Should -Contain "hab-sup(MR): Starting gossip-listener on $address"
    }

    if (!$useWindowsPester5) { Stop-Supervisor }

    It "Supervisor does not start with invalid config file" {
        $supLog = New-SupervisorLogFile("supervisor_does_not_start_with_invalid_config_file")
        Set-Content -Path $supBadConfig -Force -Value "a bad confg"

        {
            Start-Supervisor -Timeout 3 -LogFile $supLog -SupArgs @( `
            "--config-files=$supBadConfig" `
            )
        } | Should -Throw
    }

    if ($useWindowsPester5) {
        AfterAll { Remove-Item -Force -Recurse -ErrorAction Ignore $supConfig }
    } else {
        Remove-Item -Force -Recurse -ErrorAction Ignore $supConfig
    }
    if (!$useWindowsPester5) { Stop-Supervisor }
}
