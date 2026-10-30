Describe "HTTP gateway" {
    $setup = {
        if ($IsWindows) {
            $test_probe_release="habitat-testing/test-probe/0.1.0/20230620183252"
        } else {
            $test_probe_release="habitat-testing/test-probe/0.1.0/20230621013852"
        }
        $supLog = New-SupervisorLogFile("test_health_check_output_of_http_gateway")
        Start-Supervisor -LogFile $supLog -Timeout 45 -SupArgs @( `
        "--health-check-interval=10"
        ) | Out-Null
        Load-SupervisorService $test_probe_release
        Wait-Release -Ident $test_probe_release
    }
    if ($useWindowsPester5) {
        BeforeAll $setup
    } else {
        . $setup
    }

    AfterAll {
        Unload-SupervisorService -PackageName habitat-testing/test-probe -Timeout 30
        Stop-Supervisor
    }

    Context "with a service with a health-check hook" {
        $setup = {
            # test-probe has a long init hook, and we want
            # to let the health-check hoo
            Start-Sleep 20
        }
        if ($useWindowsPester5) {
            BeforeAll $setup
        } else {
            . $setup
        }

        It "returns output of the hook when queried" {
            $stdout = (Invoke-WebRequest "http://localhost:9631/services/test-probe/default/health" | ConvertFrom-Json).stdout
            $stdout | Should -MatchExactly "Running health_check hook: habitat-testing/test-probe"
        }

        It "returns status of the hook when queried" {
            $status = (Invoke-WebRequest "http://localhost:9631/services/test-probe/default/health" | ConvertFrom-Json).status
            $status | Should -MatchExactly "OK"
        }

        It "returns status of the hook from services" {
            $status = (Invoke-WebRequest "http://localhost:9631/services" | ConvertFrom-Json)[0].health_check
            $status | Should -MatchExactly "Ok"
        }
    }

    Context "with a service that changes status" {
        $setup = {
            Set-Content -Path "/hab/pkgs/$test_probe_release/health_exit" -Value 1
            Start-Sleep 30
        }
        if ($useWindowsPester5) {
            BeforeAll $setup
        } else {
            . $setup
        }

        It "returns status of the hook when queried" {
            $status = (Invoke-WebRequest "http://localhost:9631/services/test-probe/default/health" | ConvertFrom-Json).status
            $status | Should -MatchExactly "WARNING"
        }

        It "returns status of the hook from services" {
            $status = (Invoke-WebRequest "http://localhost:9631/services" | ConvertFrom-Json)[0].health_check
            $status | Should -MatchExactly "Warning"
        }
    }
}
