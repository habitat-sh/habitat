$etcPath = "/hab/etc"
$configPath = "$etcPath/cli.toml"
$ctlTlsPath = "/hab/cache/keys/ctl"

Describe "ctl gateway TLS" {
    BeforeAll {
        Remove-Item -Force -Recurse -ErrorAction Ignore $configPath
        New-Item -ItemType directory -Force -Path $etcPath
        Remove-Item -Force -Recurse -ErrorAction Ignore $ctlTlsPath
    }

    It "ctl gateway startup fails without having generated a key and certificate" {
        {
            Invoke-NativeCommand hab sup run --ctl-server-certificate --ctl-server-key
        } | Should -Throw
    }

    It "ctl gateway key and certificate generation" {
        Invoke-NativeCommand hab sup secret generate-tls --subject-alternative-name localhost
    }

    It "ctl gateway TLS connection fails without TLS enabled client" {
        $supLog = New-SupervisorLogFile("ctl_gateway_tls_connection_fails_without_tls_enabled_client")
        Start-Supervisor -LogFile $supLog -Timeout 45 -SupArgs @( `
                "--ctl-server-certificate", `
                "--ctl-server-key"
        )
        {
            Invoke-NativeCommand hab svc status
        } | Should -Throw
    }

    It "ctl gateway TLS connection" {
        $supLog = New-SupervisorLogFile("ctl_gateway_tls_connection")
        Start-Supervisor -LogFile $supLog -Timeout 45 -SupArgs @( `
                "--ctl-server-certificate", `
                "--ctl-server-key"
        )
        Set-Content -Path $configPath -Value "ctl_server_ca_certificate='$ctlTlsPath'`nlisten_ctl='localhost'"
        Invoke-NativeCommand hab svc status
    }

    It "ctl gateway TLS connection fails with wrong SNI" {
        $supLog = New-SupervisorLogFile("ctl_gateway_tls_connection_fails_with_wrong_sni")
        Start-Supervisor -LogFile $supLog -Timeout 45 -SupArgs @( `
                "--ctl-server-certificate", `
                "--ctl-server-key"
        )
        Set-Content -Path $configPath -Value "ctl_server_ca_certificate='$ctlTlsPath'`nlisten_ctl='localhost'"
        {
            Invoke-NativeCommand hab svc status --remote-sup=127.0.0.1
        } | Should -Throw
    }

    It "ctl gateway TLS connection fails with wrong server certificate" {
        $supLog = New-SupervisorLogFile("ctl_gateway_tls_connection_fails_with_wrong_server_certificate")
        Start-Supervisor -LogFile $supLog -Timeout 45 -SupArgs @( `
                "--ctl-server-certificate", `
                "--ctl-server-key"
        )

        # Get the current certificate path
        $oldCrt = Get-ChildItem "$ctlTlsPath/*.crt.pem" | Select-Object -Last 1

        # Generate a new certificate and private key
        Invoke-NativeCommand hab sup secret generate-tls --subject-alternative-name localhost

        # Fails when using the new certificate
        {
            Invoke-NativeCommand hab svc status
        } | Should -Throw

        # Succeeds when using the new certificate
        Set-Content -Path $configPath -Value "ctl_server_ca_certificate='$oldCrt'"
    }

    It "ctl gateway mutual TLS connection fails without TLS enabled client" {
        $supLog = New-SupervisorLogFile("ctl_gateway_mutual_tls_connection_fails_without_tls_enabled_client")
        Start-Supervisor -LogFile $supLog -Timeout 45 -SupArgs @( `
                "--ctl-server-certificate", `
                "--ctl-server-key", `
                "--ctl-client-ca-certificate"
        )
        Set-Content -Path $configPath -Value "ctl_server_ca_certificate='$ctlTlsPath'`nlisten_ctl='localhost'"
        {
            Invoke-NativeCommand hab svc status
        } | Should -Throw
    }

    It "ctl gateway mutual TLS connection" {
        $supLog = New-SupervisorLogFile("ctl_gateway_mutual_tls_connection")
        Start-Supervisor -LogFile $supLog -Timeout 45 -SupArgs @( `
                "--ctl-server-certificate", `
                "--ctl-server-key", `
                "--ctl-client-ca-certificate"
        )
        Set-Content -Path $configPath -Value "ctl_server_ca_certificate='$ctlTlsPath'`nctl_client_key='$ctlTlsPath'`nctl_client_certificate='$ctlTlsPath'`nlisten_ctl='localhost'"
        Invoke-NativeCommand hab svc status
    }

    It "ctl gateway TLS connection fails with wrong client certificate" {
        $supLog = New-SupervisorLogFile("ctl_gateway_tls_connection_fails_with_wrong_client_certificate")
        Start-Supervisor -LogFile $supLog -Timeout 45 -SupArgs @( `
                "--ctl-server-certificate", `
                "--ctl-server-key", `
                "--ctl-client-ca-certificate"
        )

        # Get the current certificate path
        $oldCrt = Get-ChildItem "$ctlTlsPath/*.crt.pem" | Select-Object -Last 1
        $oldKey = Get-ChildItem "$ctlTlsPath/*.key.pem" | Select-Object -Last 1

        # Generate a new certificate and private key
        Invoke-NativeCommand hab sup secret generate-tls --subject-alternative-name localhost

        # Fails when using the new key and certificate
        Set-Content -Path $configPath -Value "ctl_server_ca_certificate='$ctlTlsPath'`nctl_client_key='$ctlTlsPath'`nctl_client_certificate='$ctlTlsPath'`nlisten_ctl='localhost'"
        {
            Invoke-NativeCommand hab svc status
        } | Should -Throw

        # Failes when using the old key and new certificate
        Set-Content -Path $configPath -Value "ctl_server_ca_certificate='$oldCrt'`nctl_client_key='$ctlTlsPath'`nctl_client_certificate='$oldCrt'`nlisten_ctl='localhost'"
        {
            Invoke-NativeCommand hab svc status
        } | Should -Throw

        # Succeeds when using the old key and old certificate
        Set-Content -Path $configPath -Value "ctl_server_ca_certificate='$oldCrt'`nctl_client_key='$oldKey'`nctl_client_certificate='$oldCrt'`nlisten_ctl='localhost'"
        Invoke-NativeCommand hab svc status
    }

    It "ctl gateway TLS connection with custom SNI" {
        Invoke-NativeCommand hab sup secret generate-tls --subject-alternative-name a_test_server_name
        $supLog = New-SupervisorLogFile("ctl_gateway_tls_connection_with_custom_sni")
        Start-Supervisor -LogFile $supLog -Timeout 45 -SupArgs @( `
                "--ctl-server-certificate", `
                "--ctl-server-key"
        )

        Set-Content -Path $configPath -Value "ctl_server_ca_certificate='$ctlTlsPath'`nctl_client_key='$ctlTlsPath'`nctl_client_certificate='$ctlTlsPath'"
        {
            Invoke-NativeCommand hab svc status
        } | Should -Throw

        Set-Content -Path $configPath -Value "ctl_server_ca_certificate='$ctlTlsPath'`nctl_client_key='$ctlTlsPath'`nctl_client_certificate='$ctlTlsPath'`nctl_server_name_indication='the_wrong_server_name'"
        {
            Invoke-NativeCommand hab svc status
        } | Should -Throw

        Set-Content -Path $configPath -Value "ctl_server_ca_certificate='$ctlTlsPath'`nctl_client_key='$ctlTlsPath'`nctl_client_certificate='$ctlTlsPath'`nctl_server_name_indication='a_test_server_name'"
        Invoke-NativeCommand hab svc status
    }

    AfterEach {
        Stop-Supervisor
    }
}
