$etcPath = "/hab/etc"
$configPath = "$etcPath/cli.toml"
$ctlSecretPath = "/hab/sup/default/CTL_SECRET"

Describe "reading from supervisor and service config files" {
    BeforeAll {
        Remove-Item -Force -Recurse -ErrorAction Ignore $configPath
        New-Item -ItemType directory -Force -Path $etcPath
        Start-Supervisor -Timeout 45
        $script:ctlSecret = Get-Content -Path $ctlSecretPath
    }

    It "CLI succeeds with secret from file" {
        Invoke-NativeCommand hab svc status
    }


    It "CLI fails without secret set" {
        Remove-Item -Force -Recurse -ErrorAction Ignore $ctlSecretPath
        {
            Invoke-NativeCommand hab svc status
        } | Should -Throw
    }

    It "CLI succeeds with secret set in config file" {
        Set-Content -Path $configPath -Value "ctl_secret='$ctlSecret'"
        Invoke-NativeCommand hab svc status
    }

    AfterAll {
        Stop-Supervisor
    }
}
